use structopt::StructOpt;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use schemars::JsonSchema;

#[allow(unused_imports)]
use log::{info, error};

use crate::cli;
use crate::rest;


#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Default)]
pub struct StatusResponse {
    pub stdout: String,
    pub stderr: String,
    pub version: String,
    pub build_date: String,
    pub status_code: i32,
}

#[allow(dead_code)]
pub fn new_status_response() -> StatusResponse { 
    let resp = StatusResponse::default();
    resp
}


/// Commands that are exposed to the command line are listed here.
/// Each CLI (command line interface) command has an associated entry
/// in the enum ConsoleCommand and a struct definition which defines
/// the arguments for the command. Note for the struct for the command
/// parameters, Option<T> is optional, type T is required though you
/// can specify default="value". structopt will convert the string
/// representation to the corresponding type T.
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "console", about = "subcommands for nethost console")]
pub enum ConsoleCommand {
    #[structopt(name = "version")]
    Version(VersionCommand),
}

/// If we wanted to have arguments for the command we would put them here
#[derive(StructOpt, Serialize, Deserialize, JsonSchema, Clone, Debug, Default)]
pub struct VersionCommand {
}


#[allow(dead_code)]
/// Each CLI command has an arm defined in this function calling the
/// implementation for the command passing in the arguments as needed.
pub fn run_command(console_command: &ConsoleCommand, _opt: cli::Opt) -> cli::CommandResult {
    let net_host = cli::api_server();
    let apiport = cli::api_port();

    let result = match console_command {
        ConsoleCommand::Version(_command) => {
            let url = format!("/console/version");
            let result = rest::get::<String>(&net_host, apiport, &url, false);
            rest::print_string(result)
        },
    };
    result
}
