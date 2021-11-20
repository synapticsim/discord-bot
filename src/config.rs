use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub token: String,
	pub app_id: u64,
}

impl Config {
	pub fn try_load_from(path: impl AsRef<Path>) -> Result<Config> {
		let data = std::fs::read_to_string(path)?;
		serde_json::from_str(&data).map_err(Into::into)
	}
}
