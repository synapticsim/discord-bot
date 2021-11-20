#![feature(async_closure)]

use anyhow::Result;
use serenity::prelude::*;

use crate::{config::Config, context::BotContext};

mod config;
mod context;

#[tokio::main]
async fn main() -> Result<()> {
	let config = Config::try_load_from(std::env::args().nth(1).unwrap_or_else(|| "config.json".to_string()))?;

	let mut client = Client::builder(&config.token)
		.application_id(config.app_id)
		.event_handler(BotContext::new(&config))
		.await?;

	if let Err(err) = client.start().await {
		println!("Error: {:?}", err);
	}

	Ok(())
}
