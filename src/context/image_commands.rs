use serenity::{async_trait, model::prelude::*, prelude::*, utils::Color};

use crate::context::Command;

pub const IMAGE_COMMANDS: &[ImageCommand] = &[ImageCommand {
	command: "decals",
	image: "https://media.discordapp.net/attachments/649343789747535887/911498321762332693/unknown.png",
}];

pub struct ImageCommand {
	pub command: &'static str,
	pub image: &'static str,
}

#[async_trait]
impl Command for ImageCommand {
	async fn execute(&self, ctx: &Context, message: &Message) {
		if let Err(err) = message
			.channel_id
			.send_message(&ctx.http, |m| {
				m.embed(|e| e.color(Color::new(0x05a6ff)).image(self.image))
			})
			.await
		{
			println!("Error responding to command `{}`: {}", self.command, err)
		}
	}
}
