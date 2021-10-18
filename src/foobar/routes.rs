//use rocket_contrib::json::Json;
use rocket_okapi::openapi;

use rocket::State;

use crate::db::Db;
use crate::rest::ApiResponse;
//use super::UploadCommand;

/**
Each endpoint function we define here should also be referenced in http_server.rs
in order to expose the endpoint in the http server.
*/

// PUT is idempotent, repeated calls return same value
// whereas POST is not idempotent
// #[serde(rename_all = "camelCase")]

#[openapi]
#[get("/foobar")]
/// Run a command phase_value of phase_shifter either by serial number or unitname
pub fn foobar(_db: State<Db>) -> ApiResponse {
    // db is optional. It is mechanism if we need to ensure
    // that no concurrent access occurs to shared state data.
    // For shared state used by all http requests we have
    // a globally locked data structure.
    //let mut db = db.lock().unwrap();
    println!("hi");

    //let result = db.foobar.list(command.path.clone());
    //rest::api_response(result)
    let b: Result<bool, String> = Ok(true);
    crate::rest::api_response(b)
}

#[openapi]
#[get("/foobar/list/<path>")]
/// Run a command phase_value of phase_shifter either by serial number or unitname
pub fn list(path: String, _db: State<Db>) -> ApiResponse {
    // For shared state used by all http requests we have
    // a globally locked data structure.
    // db is optional. It is used to store data for the
    // http server. This is available on each incoming client
    // request to read/write so it must be guared by a mutex
    // to ensure exclusive access.
    // For read access:
    // let db = db.lock().unwrap();
    // For write access:
    // let mut db = db.lock().unwrap();

    println!("{}", path);

    //let result = db.foobar.list(command.path.clone());
    //rest::api_response(result)
    let mut paths = vec![];
    paths.push(String::from("a.txt"));
    paths.push(String::from("b.txt"));
    paths.push(String::from("c.txt"));

    let b: Result<Vec<String>, String> = Ok(paths);
    crate::rest::api_response(b)
}

/*
/// Upload some data and store in a file
#[openapi]
#[post("/foobar/upload", format = "json", data="<command>")]
pub fn upload(
    command: Json<ListCommand>,
   _db: State<Db>
) -> ApiResponse {

    println!("{:?}", command);
    let b: Result<bool,String> = Ok(true);
    crate::rest::api_response(b)
}
 */
