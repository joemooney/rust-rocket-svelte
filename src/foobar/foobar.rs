
use super::FoobarResult;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]  // may not use all these logging calls
use log::{debug, error, info};

/// State information for Foobar resource
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Default)]
pub struct Foobar {
    /// example field we may store for some global Foobar instance
    pub id: String,
}

/// This is the implementation for Foobar functionality
/// that is present on the server side.
impl Foobar {
    pub fn new(id: &str) -> Self {
        let foobar = Foobar {
            id: String::from(id),
        };
        foobar
    }

    pub fn list(&mut self, path: String) -> FoobarResult<Vec<String>> {
        info!("listing directory: {}", path);
        Ok(vec!["/a/b/c".to_string()])
    }

    /// Upon loading state.json we restore information as desired
    pub fn init(&mut self, restore: Foobar) {
        self.id = restore.id;
    }

}
