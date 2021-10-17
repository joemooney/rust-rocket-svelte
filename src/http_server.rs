use rocket::Rocket;
//use rocket::get;
//use rocket_contrib::json::Json;
//use rocket_okapi::{openapi, routes_with_openapi, JsonSchema};
use rocket_contrib::json::JsonValue;
use rocket_contrib::serve::StaticFiles;
use rocket_okapi::routes_with_openapi;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

use std::env;
use std::sync::Mutex;

use crate::cli;
use crate::db::GlobalState;
use crate::foobar;

// for base route api
#[get("/diskspace")]
fn diskspace() -> String {
    format!("Rust says you have lots of disk space")
}

// for base route api
#[get("/status")]
fn status() -> String {
    format!("Rust says your status is excellent")
}

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

/*
fn mount_rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/hello", routes![hello, message])
        .mount("/admin", routes![status, diskspace])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")),
        )
}
*/

/**
Each endpoint has an associated function defined in the corresponding module routes.rs file
*/
/// Launch Rocket HTTP Server
pub fn build_app(opt: cli::Opt) -> Rocket {
    env::set_var("ROCKET_PORT", opt.port.to_string());
    let global_state = Mutex::new(GlobalState::new(opt));

    let routes = routes_with_openapi![
        foobar::list,
        // foobar::directory,
    ];

    rocket::ignite()
        .manage(global_state)
        .mount("/hello", routes![hello, message])
        .mount("/admin", routes![status, diskspace])
        .mount("/api/", routes)
        .mount(
            "/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")),
        )
}
