#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
//#[macro_use]
//extern crate serde_derive;

// use rocket::request::Form;
// use rocket::http::{Cookie, Cookies};
// use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use rocket_contrib::serve::StaticFiles;

// for base route api
#[get("/")]
fn hello() -> String {
    println!("Hello, from Rust");
    format!("Hello, from Rust")
}

#[get("/joe")]
fn message() -> JsonValue {
    json!({ "result" : "success",
            "message" : "Hi from Rust!"
    })
}

fn mount_rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/hello", routes![hello, message])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")),
        )
}

fn main() {
    mount_rocket().launch();
}
