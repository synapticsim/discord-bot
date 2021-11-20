use serenity::{async_trait, model::prelude::*, prelude::*, utils::Color};

use crate::context::Command;

pub const SIMPLE_COMMANDS: &[SimpleCommand] = &[SimpleCommand {
	command: "when",
	title: "When does the A22X release?",
	response: "We have no release date or development deadlines set for the A22X at the moment as our team consists \
	           purely of volunteers dedicating their free time to this project.",
}];

pub struct SimpleCommand {
	pub command: &'static str,
	pub title: &'static str,
	pub response: &'static str,
}

#[async_trait]
impl Command for SimpleCommand {
	async fn execute(&self, ctx: &Context, message: &Message) {
		if let Err(err) = message
			.channel_id
			.send_message(&ctx.http, |m| {
				m.embed(|e| {
					e.color(Color::new(0x05a6ff))
						.title(self.title)
						.description(self.response)
				})
			})
			.await
		{
			println!("Error responding to command `{}`: {}", self.command, err)
		}
	}
}
