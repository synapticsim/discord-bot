use serenity::{async_trait, model::prelude::*, prelude::*, utils::Color};

use crate::context::Command;

pub struct ImageCommand {
	pub image: String,
}

#[async_trait]
impl Command for ImageCommand {
	async fn execute(&self, ctx: &Context, message: &Message) {
		if let Err(err) = message
			.channel_id
			.send_message(&ctx.http, |m| {
				m.embed(|e| e.color(Color::new(0x05a6ff)).image(&self.image))
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
		let image = input
			.args
			.get("url")
			.ok_or(anyhow::anyhow!("No image url provided"))?
			.as_str()
			.ok_or(anyhow::anyhow!("Image url must be string"))?
			.to_string();

		Ok(Self { image })
	}
}
