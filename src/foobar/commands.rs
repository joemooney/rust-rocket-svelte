use super::Foobar;
use crate::cli;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

/// Commands exposed to the command line are listed here.
/// Each CLI (command line interface) command has an associated entry
/// in the enum FoobarCommand and optionally a struct definition which defines
/// the arguments for the command.

/// For the struct for the command parameters follow the structopt crate spec
/// For example, Option<T> is an optional argument.
/// You can specify default="value" and structopt will convert the string
/// representation to the corresponding type.

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "foobar", about = "subcommands for foobar")]
pub enum FoobarCommand {
    #[structopt(name = "list")]
    List(ListCommand),
}

#[derive(StructOpt, Serialize, Deserialize, JsonSchema, Clone, Debug, Default)]
/// List the contents of a directory path
pub struct ListCommand {
    pub path: String,
}

#[allow(dead_code)]
/// Bypass the REST API and invoke commands directly
/// Each CLI command has an arm defined in this function calling the
/// implementation for the command passing in the arguments as needed.
/// Whether this is possible for a command depends on circumstances.
pub fn run_local_command(foobar_command: &FoobarCommand, _opt: &cli::Opt) -> cli::CommandResult {
    let mut foobar = Foobar::new("foobar cli");
    match foobar_command {
        FoobarCommand::List(command) => {
            if let Err(e) = foobar.list(command.path.clone()) {
                return Err(format!("could not execute list command: {}", e));
            }
        }
    }
    Ok(())
}

/// Invode the REST endpoint via a CLI command passing in the arguments as needed.
pub fn run_command(foobar_command: &FoobarCommand, _opt: &cli::Opt) -> cli::CommandResult {
    let api_server = cli::api_server();
    let api_port = cli::api_port();

    match foobar_command {
        FoobarCommand::List(command) => {
            let url = format!("/foobar/list");
            println!("listing foobar: {}:{}{}", api_server, api_port, url);
            let result = crate::rest::get_json::<ListCommand, Vec<String>>(
                &api_server,
                api_port,
                &url,
                command.to_owned(),
                false,
            );
            let _ = crate::rest::print_result(result);
        }
    }
    Ok(())
}
