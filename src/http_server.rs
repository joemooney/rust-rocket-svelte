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

// This is using /admin/diskspace route
#[get("/diskspace")]
fn diskspace() -> String {
    format!("Rust says you have lots of disk space")
}

// This is using /admin/status route
#[get("/status")]
fn status() -> String {
    format!("Rust says your status is excellent")
}

// This is using /hello route
#[get("/")]
fn hello() -> String {
    println!("Hello, from Rust");
    format!("Hello, from Rust")
}

// This is using /hello/joe route
#[get("/joe")]
fn message() -> JsonValue {
    json!({ "result" : "success",
            "message" : "Hi from Rust!"
    })
}

/**
Each endpoint has an associated function defined in the corresponding module routes.rs file
*/
/// Launch Rocket HTTP Server
pub fn build_app(opt: cli::Opt) -> Rocket {
    env::set_var("ROCKET_PORT", opt.port.to_string());
    let global_state = Mutex::new(GlobalState::new(opt));

    let openapi_routes = routes_with_openapi![
        foobar::foobar,
        foobar::list,
        // foobar::directory,
    ];

    rocket::ignite()
        .manage(global_state)
        .mount("/hello", routes![hello, message])
        .mount("/admin", routes![status, diskspace])
        // routes for which we have the #[openapi] attribute specified
        .mount("/", openapi_routes)
        // http:<hostname>:<port>/api presents a web page
        // with all the openapi endpoints so you can try them out.
        .mount(
            "/api/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        // The svelte endpoint is available at:
        // http://<hostname>:<port>/
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")),
        )
}
