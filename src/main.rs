#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::response::Stream;
use rocket_contrib::json::JsonValue;
//  Compiles fine but rls highlights function with: called `Option::unwrap()` on a `None` value
#[get("/foobar/<a>/<b>")]
fn foobar(a: usize, b: usize) {
    println!("foobar {} {}", a, b);
}

// use rocket::request::Form;
// use rocket::http::{Cookie, Cookies};
// use rocket_contrib::json::Json;
use std::process::{ChildStdout, Command, Stdio};
//use std::{thread, time};

// for base route api
#[get("/journalctl2")]
fn journalctl2() -> Result<Stream<ChildStdout>, std::io::Error> {
    println!("journalctl starting");
    let child = Command::new("tail")
        .arg("-f")
        .arg("/tmp/foo.txt")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let response = Stream::from(child.stdout.unwrap());
    println!("journalctl returning response");
    Ok(response)
}

mod cli;
mod console;
mod db;
mod foobar;
mod http_server;
mod rest;

#[allow(dead_code)]
const BUILD_DATE: &'static str = "20211006_171846";

// return a number of lines from a given offset from the journalctl
#[get("/journalctl/<offset>/<number>")]
fn journalctl(offset: usize, number: usize) -> String {
    println!(
        "journalctl getting lines {} .. {}",
        offset,
        offset + number - 1
    );
    let mut s = String::new();
    for n in 0..number {
        s.push_str(&format!("line {}\n", offset + n + 1));
    }
    s
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
        .mount(
            "/admin",
            routes![status, diskspace, journalctl, journalctl2],
        )
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")),
        )
}
*/

fn main() {
    let opt = cli::parse_args();

    if let Some(subcommand) = &opt.subcommand {
        console::log_output(opt.log_level);
        cli::execute_subcommand(subcommand, &opt);
    } else {
        http_server::build_app(opt).launch();
    }
}
