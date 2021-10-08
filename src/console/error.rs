use thiserror::Error;
use std::fmt;
//use rocket::http::Status;
//use serde::Serialize;


#[allow(dead_code)]
pub type ConsoleResult<T> = std::result::Result<T, ConsoleError>;

#[derive(Error, Debug)]
pub enum ConsoleError {
    //#[error("failed to run command {0}:{1}{2} {3}")]
    //RestApiError(String, String, String, String),
    //#[error("failed to send protobuf message to radio {0}: {1}")]
    //SendProtobufError(String, String),
}

#[derive(Error, Debug)]
pub struct CommandError {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

/*
/// Convert a CommandError into a string for error reporting
impl ToString for CommandError {
    fn to_string(&self) -> String {
        format!("status:{} stdout:<{}> stderr:<{}>", self.status, self.stdout, self.stderr)
    }
}
*/


/// Convert a CommandError into a string for error reporting
impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CommandError status:{} stdout:<{}> stderr:<{}>", self.status, self.stdout, self.stderr)
    }
}


/*

 #[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}
 */