// #![feature(proc_macro_hygiene, decl_macro)]

use structopt::StructOpt;
use log::LevelFilter;

use lazy_static::lazy_static;
use std::sync::Mutex;

#[allow(unused_imports)]  // may not use all logging functions
use log::{info, error};

// include all the modules containing subcommands
use crate::foobar;

pub const DATE_FORMAT_STR: &'static str = "%Y%m%d_%H%M%S";
pub type CommandResult = Result<(), String>;

// Server port providing REST API to CLI
lazy_static! { pub static ref API_PORT: Mutex<u32> = Mutex::new(37000u32); }
pub fn api_port() -> u32 { *API_PORT.lock().unwrap() }
pub fn set_api_port(port: u32) { *API_PORT.lock().unwrap() = port; }


// Server name/ip providing REST API to CLI
lazy_static! { pub static ref API_SERVER: Mutex<String> = Mutex::new(String::new()); }
pub fn api_server() -> String { String::from(&*API_SERVER.lock().unwrap()) }
pub fn set_api_server(server: String) { 
    if server == "localhost" {
        *API_SERVER.lock().unwrap() = hostname();
    } else {
        *API_SERVER.lock().unwrap() = server; 
    }
}

// Location of persistent state stored in JSON file
lazy_static! { pub static ref STATE_JSON: Mutex<String> = Mutex::new(format!("./state.json")); }
pub fn state_json() -> String { let s = &*STATE_JSON.lock().unwrap(); s.clone() }
#[allow(dead_code)]
pub fn set_state_json(state_json: String) {
    *STATE_JSON.lock().unwrap() = state_json;
}

pub fn hostname() -> String {
    String::from(&gethostname::gethostname().into_string().unwrap())
}

#[allow(dead_code)]
pub fn pid() -> u32 {
    std::process::id()
}

pub fn parse_args() -> Opt {
	let opt = Opt::from_args();

    // Global static variables accessible via Arc<Mutex<T>>
    set_api_port(opt.port);
    set_api_server(opt.server.clone());
	opt
}


//#[derive(StructOpt, Debug, Clone, Default)]  --> LogFilter has no Default
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "basic")]
pub struct Opt {

    /// Version
    #[structopt(short, long)]
    pub version: bool,

    /// Run the command locally without using the http server (only for debugging purposes)
    #[structopt(long)]
    pub local: bool,

    /// Host of http server
    #[structopt(long, default_value = "localhost")]
    pub server: String,

    /// Port for rust http server
    #[structopt(short, long, env = "API_PORT", default_value = "37000")]
    pub port: u32,

    /// log level for output to console (error, warn, info, debug, trace)
    #[structopt(long, default_value = "Info")]
    pub log_level: LevelFilter,

    /// SUBCOMMANDS
    #[structopt(subcommand)]
    pub subcommand: Option<SubCommand>,
}

/// Each subcommand is listed here
/// Nested subcommands are defined within the respective subcommand
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "cli", about = "subcommands for cli")]
pub enum SubCommand {
    #[structopt(name = "foobar")]
    Foobar(foobar::FoobarCommand),
}

/// Execute a command provided on the command line.
/// This is called from main.rs
pub fn execute_subcommand(subcommand: &SubCommand, opt: &Opt) {
    let _result = match subcommand {
        SubCommand::Foobar(command) => {
            foobar::run_command(&command, opt)
        }
    };
    // TODO: figure out what to do here and whether to pass a result
    /*
    if let Err(err) = result {
        error!("{}", err)
    }
    */
}
