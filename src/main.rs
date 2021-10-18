#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::response::Stream;
use rocket_contrib::json::JsonValue;

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
fn main() {
    let opt = cli::parse_args();

    if let Some(subcommand) = &opt.subcommand {
        console::log_output(opt.log_level);
        cli::execute_subcommand(subcommand, &opt);
    } else {
        http_server::build_app(opt).launch();
    }
}
