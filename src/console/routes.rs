use rocket::get;
use rocket_contrib::json::Json;
use rocket_okapi::openapi;


// use crate::db::Db;
//use std::fs;
//use std::os::unix::process::CommandExt;
//use std::process::Command;
// use protobuf::well_known_types::Api;
// use rocket::State;
//use rocket::http::Status;

//use std::path::Path;
//use std::path::PathBuf;
//use crate::rest::{self, ApiResponse};

#[allow(unused_imports)]  // may not use all logging functions
use log::{debug, error, info, warn, LevelFilter};

#[allow(dead_code)]
#[openapi]
#[get("/")]
/// Used to check root endpoint is available and console http server is running.
pub fn console_running() -> Json<bool> {
    info!("[/] console http server is running");
    Json(true)
}

#[allow(dead_code)]
#[openapi]
#[get("/console/version")]
/// Used to check root endpoint is available and console http server is running.
pub fn console_version() -> Json<String> {
    info!(
        "[/console/version] Version is: {}",
        env!("CARGO_PKG_VERSION").to_string()
    );
    return Json(format!("{} {}", env!("CARGO_PKG_VERSION"), crate::BUILD_DATE));
}

/*
#[openapi]
#[get("/console/loglevel")]
/// Retreieve the Log Level of the consolse
pub fn console_loglevel(db: State<Db>) -> Json<String> {
    let db = db.lock().unwrap();
    info!("Queried for log level");
    return Json(format!("{}", db.log_level));
}

#[openapi]
#[get("/console/loglevel/<log_level>")]
/// Set the Log Leve of the console: trace, debug, info, warn, error, off
pub fn console_set_loglevel(log_level: String, db: State<Db>) -> Json<String> {
    let mut db = db.lock().unwrap();
    db.log_level = match log_level.as_str() {
        "off" => LevelFilter::Off,
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _       => {
            return Json(format!("Invalid log level{}, valid values are: off, trace, debug, info, warn, error. Keeping log level at: {}", log_level, db.log_level))
        },
    };
    info!("Set for log level: {}", db.log_level);
    super::logging::set_log_level(db.log_level, &db.logger);
    return Json(format!("{}", db.log_level));
}
#[openapi]
#[get("/console/stop")]
/// WARNING: This terminates the http server.
pub fn console_exit(_db: State<Db>) {
    warn!("[/console/stop] exiting!");
    std::process::exit(3);
}

#[openapi]
#[get("/console/status")]
/// Used to status of console http server.
pub fn console_status(_db: State<Db>) -> ApiResponse {
    let version = env!("CARGO_PKG_VERSION").to_string();
    let build_date = String::from(crate::BUILD_DATE);
    let ok = format("{} {} Running ok", version, build_date);
    rest::api_response(Ok(ok))
}
*/

//#[post("/stop", format = "json", data = "<req>")]

