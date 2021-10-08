use crate::console;
use chrono::{DateTime, Local};
use crate::foobar::Foobar;
use crate::cli;
use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::time::SystemTime;

use std::sync::Mutex;
use log4rs::Handle;
use log::LevelFilter;
use super::{DbResult, DbError};
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use log::{info, error};


#[derive(Serialize, Deserialize)]
pub struct GlobalState {
	#[serde(skip, default="dummy_log_level")]
    pub log_level: LevelFilter,
	#[serde(skip)]
    pub logger: Option<Handle>,
    pub foobar: Foobar,
}

impl Default for GlobalState {
	fn default() -> Self {
		Self {
			log_level: dummy_log_level(),
			logger: None,
			foobar: Foobar::default(),
		}
	}
}

fn dummy_log_level() -> LevelFilter { LevelFilter::Info }

/// The GlobalState is the state shared across this application.
impl GlobalState {
    pub fn new(opt: cli::Opt) -> Self {
        let mut db = Self {
            log_level: opt.log_level,
            logger: Some(console::log_output(opt.log_level)),
            foobar: Foobar::new("anything"),
        };
		// Load previous state in state.json
		// ignore if state.json doesn't exist, else load
		db.init();
		db
    }

    /// Read state from JSON file stored on disk
    pub fn load_state_from_disk(&mut self) -> DbResult<Option<GlobalState>> {
        let mut data = String::new();
		let state_json = cli::state_json();
		match std::path::Path::new(&state_json).exists() {
			false => {
				Ok(None)
			}
			true => {
				match File::open(&state_json) {
					Ok(mut file) => match file.read_to_string(&mut data) {
						Ok(_) => match serde_json::from_str::<GlobalState>(&data) {
							Ok(s) => Ok(Some(s)),
							Err(e) => Err(DbError::LoadError(format!(
								"{} is not well-formatted: {}", state_json, e))),
						},
						Err(e) => Err(DbError::LoadError(format!(
							"Unable to load {}: {}", state_json, e))),
					},
					Err(e) => Err(DbError::LoadError(format!(
						"Unable to read {}: {}", state_json, e))),
				}
			}
		}
    }

	/// Load previous state stored in state.json
	/// ignore if state.json doesn't exist
	/// Then scan this computer for attached devices
	pub fn init(&mut self) {
		match self.load_state_from_disk() {
			Ok(Some(db)) => {
				self.foobar.init(db.foobar)
			},
			Ok(None) => {
			},
			Err(err) => {
				error!("{}", err);
			},
		}
		self.save().expect("Unable to save state.json");
	}

	#[allow(dead_code)]
    /// Try to save state to a file
    pub fn try_save(&mut self) -> bool {
		if let Err(err) = self.save() {
			error!("{}", err);
			false
		} else { 
			true
		}
    }

    /// Save state to disk
    pub fn save(&mut self) -> DbResult<String> {
		let state_json = cli::state_json();
		let now: DateTime<Local> = SystemTime::now().into();
		let ts = now.format(cli::DATE_FORMAT_STR);
		self.save_to_file(state_json.clone())?;
		self.save_to_file(format!("{}.{}", state_json, ts))
    }


    /// Write out state to a file
    pub fn save_to_file(&mut self, path: String) -> DbResult<String> {
        match serde_json::to_string_pretty(&self) {
            Ok(serialized) => match File::create(&path) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(serialized.as_bytes()) {
                        Err(DbError::SaveError(format!(
                            "Unable to write json to {}: {}", path, e)))
                    } else {
                        info!("Saved {}", path);
                        Ok(serialized)
                    }
                }
                Err(e) => Err(DbError::SaveError(format!(
                    "Unable to write {}: {}", path, e))),
            },
            Err(e) => Err(DbError::SaveError(format!(
                "Unable to serialize state: {}",
                e
            ))),
        }
    }

}
pub type Db = Mutex<GlobalState>;
