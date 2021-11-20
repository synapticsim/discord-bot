use std::{collections::HashMap, ops::Deref};

use serenity::{async_trait, model::prelude::*, prelude::*};

use crate::{
	context::{image_commands::IMAGE_COMMANDS, simple_commands::SIMPLE_COMMANDS},
	Config,
};

mod image_commands;
mod simple_commands;

#[async_trait]
trait Command: Sync {
	async fn execute(&self, ctx: &Context, message: &Message);
}

pub struct BotContext {
	commands: HashMap<&'static str, &'static dyn Command>,
}

pub struct LockedBotContext(RwLock<BotContext>);

impl Deref for LockedBotContext {
	type Target = RwLock<BotContext>;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl BotContext {
	pub fn new(_config: &Config) -> LockedBotContext {
		LockedBotContext(RwLock::new(Self {
			commands: {
				let mut h = HashMap::new();
				for s in SIMPLE_COMMANDS {
					h.insert(s.command, s as &dyn Command);
				}
				for i in IMAGE_COMMANDS {
					h.insert(i.command, i);
				}
				h
			},
		}))
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
