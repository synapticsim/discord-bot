use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct Secret {
	pub token: String,
	pub app_id: u64,
}

impl Secret {
	pub fn try_load_from(path: impl AsRef<Path>) -> Result<Self> {
		let data = std::fs::read_to_string(path)?;
		serde_json::from_str(&data).map_err(Into::into)
	}
}

#[derive(Serialize, Deserialize)]
pub struct Command {
	pub name: String,
	pub ty: String,
	pub args: Value,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
	pub commands: Vec<Command>,
}

impl Config {
	pub fn try_load_from(path: impl AsRef<Path>) -> Result<Self> {
		let data = std::fs::read_to_string(path)?;
		serde_json::from_str(&data).map_err(Into::into)
	}
}
