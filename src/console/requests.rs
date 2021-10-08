use serde::{Deserialize, Serialize};
// use std::{collections::HashMap};

// PUT is idempotent, repeated calls return same value
// whereas POST is not idempotent
// #[serde(rename_all = "camelCase")]

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
/// Upgrade server
/// Without path_override we will use haboobnas
/// Optionally use a new version of software
/// or a specific binary path
pub struct UpgradeRequest {
    pub path_override: bool,
    pub path: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
/// Execute a known command on a radio
pub struct ExecuteRequest {
    pub serial_number: String,
    pub test_net: String,
    pub test_setup: String,
    pub radio_setup: String,
    pub cmd: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Default)]
/// Execute a known command on a radio
pub struct StopRequest {
    pub auth: Option<String>,  // future auth token
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Default)]
/// Execute a known command on a radio
pub struct RebootRequest {
    pub wait_for_connection: bool,
}
