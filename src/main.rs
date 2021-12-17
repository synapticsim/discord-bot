use anyhow::Result;
use serenity::prelude::*;

use crate::{
	config::{Config, Secret},
	context::BotContext,
};

mod config;
mod context;

#[tokio::main]
async fn main() -> Result<()> {
	let secret = Secret::try_load_from(std::env::args().nth(1).unwrap_or_else(|| "secret.json".to_string()))?;
	let config = Config::try_load_from("config.json")?;

	let mut client = Client::builder(&secret.token)
		.application_id(secret.app_id)
		.event_handler(BotContext::new(config)?)
		.await?;

	if let Err(err) = client.start().await {
		println!("Error: {:?}", err);
	}

	Ok(())
}
