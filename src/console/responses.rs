//use std::{collections::HashMap};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
//use crate::radios::Radios;


/*
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Default)]
pub struct ShellResponse {
    pub serial_number: String,
    pub stdout: String,
    pub stderr: String,
    pub status_code: i32,
}
 */


#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Default)]
pub struct StatusResponse {
    pub stdout: String,
    pub stderr: String,
    pub version: String,
    pub build_date: String,
    pub status_code: i32,
}

pub fn new_status_response() -> StatusResponse { 
    let resp = StatusResponse::default();
    resp
}
