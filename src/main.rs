#![feature(decl_macro, proc_macro_hygiene)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;


mod db;
mod cli;
mod console;
mod rest;
mod foobar;
mod http_server;

#[allow(dead_code)]
const BUILD_DATE: &'static str = "20211006_171846";

fn main() {
    let opt = cli::parse_args();
     
    if let Some(subcommand) = &opt.subcommand {
        console::log_output(opt.log_level);
        cli::execute_subcommand(subcommand, &opt);
    } else {
        http_server::build_app(opt).launch();
    }
}