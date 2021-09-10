#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

// use rocket::request::Form;
use rocket::http::{Cookie, Cookies};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::{JsonValue,Json};


// for base route api
#[get("/")]
fn hello() -> String {
    println!("Hello, from Rust  \n Hit the back button to continue");
    format!("Hello, from Rust  \n Hit the back button to continue")
}

#[get("/joe")]
fn message() -> JsonValue{
    json!({ "result" : "success",
            "message" : "hi from joe"
    })
}

fn mount_rocket() -> rocket::Rocket{
    rocket::ignite()
    .mount("/hello",routes![hello, message])
    //.mount("/auth",routes![login_user,logout_user])
    //.mount("/app",routes![common])
    .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")))

}

fn main() {
    mount_rocket()
    .launch();
}
