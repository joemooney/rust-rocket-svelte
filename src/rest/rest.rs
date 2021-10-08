use serde::de;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use schemars::JsonSchema;

use okapi::openapi3::Responses;
use reqwest::StatusCode;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response;
use rocket::response::{Responder, Response};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::response::OpenApiResponder;

#[allow(unused_imports)]  // may not use all logging functions
use log::{error, info};

// Using HTTP Methods for RESTful Services
// https://www.restapitutorial.com/lessons/httpmethods.html
// POST    Create          201 (Created), 'Location' header with link to /customers/{id} containing new ID.
//                         404 (Not Found),
//                         409 (Conflict) if resource already exists..
// GET     Read            200 (OK), list of customers. Use pagination, sorting and filtering to navigate big lists.     
//                         200 (OK), single customer. 
//                         404 (Not Found), if ID not found or invalid.
// PUT     Update/Replace  405 (Method Not Allowed), unless you want to update/replace every resource in the entire collection.     200 (OK) or 204 (No Content). 404 (Not Found), if ID not found or invalid.
// PATCH   Update/Modify   405 (Method Not Allowed), unless you want to modify the collection itself.     
//                         200 (OK) or 204 (No Content). 404 (Not Found), if ID not found or invalid.
// DELETE  Delete          405 (Method Not Allowed), unless you want to delete the whole collection—not often desirable.     
//                         200 (OK). 404 (Not Found), if ID not found or invalid.


type CommandResult = Result<(), String>;

/// ApiError is a struct that gets serialized into an ApiResponse object
/// then on the client side if the ApiResponse status is not good (typically Ok)
/// then the json will deserialize into an ApiError
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
struct ApiError {
    short: String,
    long: String,
    status: u8,
}


/// ApiResponse is a struct passed back from a REST request.
/// The Json is either an ApiError or the output depending on the status.
// #[derive(Serialize, JsonSchema, Clone, Debug)]
#[derive(Clone, Debug)]

pub struct ApiResponse {
    /// deserializes to either an ApiError if status is not ok,
    /// or deserializes to the expected data type
    json: String,
    /// a http status code
    status: Status,
}

impl OpenApiResponder<'_> for ApiResponse {
    fn responses(_gen: &mut OpenApiGenerator) -> rocket_okapi::Result<Responses> {
        let responses = Responses::default();
        // let schema = gen.json_schema::<String>();
        // add_schema_response(&mut responses, 200, "text/plain", schema)?;
        //println!("Generating ApiResponse");
        Ok(responses)
    }
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.json.respond_to(&req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}

#[allow(dead_code)]
pub fn request<T: de::DeserializeOwned + std::fmt::Debug>(
    server: &str,
    port: u32,
    url: &str,
    print: bool,
) -> Result<T, reqwest::Error> {
    let url = format!("http://{}:{}{}", server, port, url);
    let resp = reqwest::blocking::get(url)?.json::<T>()?;
    if print {
        println!("{:#?}", resp);
    }
    Ok(resp)
}

#[allow(dead_code)]
pub fn post_no_json<T: de::DeserializeOwned + std::fmt::Debug>(
    server: &str,
    port: u32,
    url: &str,
    print: bool,
) -> Result<T, String> {
    let client = reqwest::blocking::Client::new();
    let url = format!("http://{}:{}{}", server, port, url);
    let resp = client.post(url).send().map_err(|e| format!("{:?}", e))?;
    handle_resp(resp, print)
}

#[allow(dead_code)]
/// Use this to POST (create) a request with a JSON body
pub fn post_request<J: Serialize, T: de::DeserializeOwned + std::fmt::Debug>(
    server: &str,
    port: u32,
    url: &str,
    json: J,
    print: bool,
) -> Result<T, String> {
    let client = reqwest::blocking::Client::new();
    let url = format!("http://{}:{}{}", server, port, url);
    let resp = client.post(url).json(&json).send().map_err(|e| format!("{:?}", e))?;
    handle_resp(resp, print)
}

#[allow(dead_code)]
pub fn put_no_json<T: de::DeserializeOwned + std::fmt::Debug>(
    server: &str,
    port: u32,
    url: &str,
    print: bool,
) -> Result<T, String> {
    let client = reqwest::blocking::Client::new();
    let url = format!("http://{}:{}{}", server, port, url);
    let resp = client.put(url).send().map_err(|e| format!("{:?}", e))?;
    handle_resp(resp, print)
}

#[allow(dead_code)]
/// Use this perform a PUT (update) request with a JSON body
pub fn put_request<J: Serialize, T: de::DeserializeOwned + std::fmt::Debug>(
    server: &str,
    port: u32,
    url: &str,
    json: J,
    print: bool,
) -> Result<T, String> {
    let client = reqwest::blocking::Client::new();
    let url = format!("http://{}:{}{}", server, port, url);
    let resp = client.post(url).json(&json).send().map_err(|e| format!("{:?}", e))?;
    handle_resp(resp, print)
}

/// Use this perform a GET (query) request with a JSON body
pub fn get_json<J: Serialize, T: de::DeserializeOwned + std::fmt::Debug>(
    server: &str,
    port: u32,
    url: &str,
    json: J,
    print: bool,
) -> Result<T, String> {
    let client = reqwest::blocking::Client::new();
    let url = format!("http://{}:{}{}", server, port, url);
    let resp = client.get(url).json(&json).send().map_err(|e| format!("{:?}", e))?;

    match resp.status() {
        StatusCode::OK => {
            let resp = resp
                .json::<T>()
                .map_err(|e| format!("failed deserializing result: {:?}", e))?;
            if print {
                println!("{:#?}", resp);
            }
            Ok(resp)
        }
        _ => {
            let err = resp
                .json::<ApiError>()
                .map_err(|e| format!("failed deserializing error: {:?}", e))?;
            // println!("error is {}", err.short);
            Err(format!("{}: {}", err.short, err.long))
        }
    }
}

#[allow(dead_code)]
/// Use this perform a GET (query) request without a JSON body
pub fn get<T: de::DeserializeOwned + std::fmt::Debug>(
    server: &str,
    port: u32,
    url: &str,
    print: bool,
) -> Result<T, String> {
    let url = format!("http://{}:{}{}", server, port, url);
    let resp = reqwest::blocking::get(url).map_err(|e| format!("{:?}", e))?;

    match resp.status() {
        StatusCode::OK => {
            let resp = resp
                .json::<T>()
                .map_err(|e| format!("failed deserializing result: {:?}", e))?;
            if print {
                println!("{:#?}", resp);
            }
            Ok(resp)
        }
        _ => {
            let err = resp
                .json::<ApiError>()
                .map_err(|e| format!("failed deserializing error: {:?}", e))?;
            // println!("error is {}", err.short);
            Err(format!("{}: {}", err.short, err.long))
        }
    }
}

pub fn handle_resp<T: de::DeserializeOwned + std::fmt::Debug>(resp: reqwest::blocking::Response, print: bool) -> Result<T, String> {
    match resp.status() {
        StatusCode::OK => {
            let resp = resp
                .json::<T>()
                .map_err(|e| format!("failed deserializing result: {:?}", e))?;
            if print {
                println!("{:#?}", resp);
            }
            Ok(resp)
        }
        _ => {
            let err = resp
                .json::<ApiError>()
                .map_err(|e| format!("failed deserializing error: {:?}", e))?;
            // println!("error is {}", err.short);
            Err(format!("{}: {}", err.short, err.long))
        }
    }
}

#[allow(dead_code)]
/// For Command Line Interface
/// Print a String received as a response from a REST endpoint call
pub fn print_string<T: std::fmt::Display, E: std::fmt::Display>(result: Result<T, E>) -> CommandResult {
    match result {
        Ok(v) => {
            println!("{}", v);
            Ok(())
        },
        Err(e) => {
            //error!("could not execute command: {}", e);
            println!("{}", e);
            Err(format!("failed to execute command: {}", e))
        }
    }
}

/// For Command Line Interface
/// Print output for a Command Line Interface result
pub fn print_result<T: Debug + Serialize, E: Debug>(result: Result<T, E>) -> CommandResult {
    match result {
        Ok(v) => {
            if let Ok(s) = serde_json::to_string_pretty(&v) {
                println!("{}", s);
            } else { 
                println!("{:?}", v);
            }
            Ok(())
        }
        Err(e) => {
            //error!("could not execute command: {:?}", e);
            Err(format!("{:?}", e))
        }
    }
}


/// Serialize object or return a error string
pub fn api_serialize<T: Serialize>(v: &T) -> String {
    let err = r#"{"error": {"short": "Failed to serialize", "long": "internal error"}}"#;
    let json = serde_json::to_string(v);
    match json {
        Ok(s) => s,
        Err(e) => {
            error_!("JSON failed to serialize: {:?}", e);
            err.to_string()
        }
    }
}

#[allow(dead_code)]
/// Return a error ApiResponse object
/// Generate a serialized JSON error response used for HTTP requests
/// This also sets the provided HTTP status code.
pub fn api_error(short: &str, long: &str, status: Status) -> ApiResponse {
    let err = ApiError {
        short: short.to_string(),
        long: long.to_string(),
        status: 1,
    };
    let json = api_serialize(&err);
    ApiResponse {
        json: json,
        status: status,
    }
}

/// Generate a false boolean Status::BadRequest response
pub fn api_fail(errmsg: &str) -> ApiResponse {
    let err = ApiError {
        short: errmsg.to_string(),
        long: String::from(""),
        status: 1,
    };
    let json = api_serialize(&err);
    ApiResponse {
        json: json,
        status: Status::BadRequest,
    }
}

#[allow(dead_code)]
/// Generate a true boolean Status::Ok response
pub fn api_pass() -> ApiResponse {
    let json = api_serialize(&true);
    ApiResponse { json, status: Status::Ok }
}

/// Generate a serialized JSON response used for rocket HTTP requests
pub fn api_ok<T: Serialize>(result: &T, status: Status) -> ApiResponse {
    let json = api_serialize(result);
    ApiResponse { json, status }
}

//  Take a Result and convert it into an ApiResponse
//  with Err mapped to ApiError
//  and Ok mapped to ApiOk
pub fn api_response<T: Serialize, E: ToString>(result: Result<T, E>) -> crate::rest::ApiResponse {
    match result {
        Ok(v) => crate::rest::api_ok(&v, Status::Ok),
        Err(e) => crate::rest::api_fail(&e.to_string()),
    }
}