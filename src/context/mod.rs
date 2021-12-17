use std::{collections::HashMap, ops::Deref};

use anyhow::Result;
use serenity::{async_trait, model::prelude::*, prelude::*};

use self::{image_command::ImageCommand, simple_command::SimpleCommand};
use crate::Config;

mod image_command;
mod simple_command;

#[async_trait]
trait Command: Sync + Send {
	async fn execute(&self, ctx: &Context, message: &Message);

	fn parse(input: crate::config::Command) -> Result<Self>
	where
		Self: Sized;
}

pub struct BotContext {
	commands: HashMap<String, Box<dyn Command>>,
}

pub struct LockedBotContext(RwLock<BotContext>);

impl Deref for LockedBotContext {
	type Target = RwLock<BotContext>;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl BotContext {
	pub fn new(config: Config) -> Result<LockedBotContext> {
		Ok(LockedBotContext(RwLock::new(Self {
			commands: {
				let mut h: HashMap<_, Box<dyn Command>> = HashMap::new();

				for command in config.commands {
					let name = command.name.clone();
					match command.ty.as_str() {
						"simple" => {
							SimpleCommand::parse(command).map(|c| h.insert(name, Box::new(c)))?;
						},
						"image" => {
							ImageCommand::parse(command).map(|c| h.insert(name, Box::new(c)))?;
						},
						_ => {},
					}
				}

				h
			},
		})))
	}
}

#[async_trait]
impl EventHandler for LockedBotContext {
	async fn message(&self, ctx: Context, message: Message) {
		if message.author.bot {
			return;
		}

		if message.content.starts_with('.') {
			if let Some(command) = message.content[1..].split_whitespace().next() {
				if let Some(command) = self.read().await.commands.get(command) {
					command.execute(&ctx, &message).await;
				}
			}
		}
	}
}
