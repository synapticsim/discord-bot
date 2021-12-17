use serenity::{async_trait, model::prelude::*, prelude::*, utils::Color};

use crate::context::Command;

pub struct SimpleCommand {
	pub title: String,
	pub response: String,
}

#[async_trait]
impl Command for SimpleCommand {
	async fn execute(&self, ctx: &Context, message: &Message) {
		if let Err(err) = message
			.channel_id
			.send_message(&ctx.http, |m| {
				m.embed(|e| {
					e.color(Color::new(0x05a6ff))
						.title(&self.title)
						.description(&self.response)
				})
			})
			.await
		{
			println!("Error responding to command: {}", err)
		}
	}

	fn parse(input: crate::config::Command) -> anyhow::Result<Self>
	where
		Self: Sized,
	{
		let title = input
			.args
			.get("title")
			.ok_or(anyhow::anyhow!("No title"))?
			.as_str()
			.ok_or(anyhow::anyhow!("Title must be string"))?
			.to_string();
		let response = input
			.args
			.get("response")
			.ok_or(anyhow::anyhow!("No response"))?
			.as_str()
			.ok_or(anyhow::anyhow!("Response must be string"))?
			.to_string();
		
		Ok(Self {
			title,
			response,
		})
	}
}
