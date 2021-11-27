use serenity::framework::standard::{macros::command, CommandResult };
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::ConfigStore;

#[command]
#[description = "Shows the current bot config"]
async fn show(ctx: &Context, msg: &Message) -> CommandResult {
	println!("calling show_config command");
	let bot_conf_lock = {
		let data_read = ctx.data.read().await;
		data_read.get::<ConfigStore>().unwrap().clone()
	};
	{
		let bot_conf = bot_conf_lock.read().await;

		let msg = msg
			.channel_id
			.send_message(&ctx.http, |m| {
				m.embed(|e| {
					e.title("Bot Configuration");
					e.fields(vec![
						("clear_command_calls", format!("{}", bot_conf.get_clear_calls()), false),
						("muted", format!("{}", bot_conf.muted()), false),
						("flood_delay", format!("{} ms", bot_conf.get_flood_delay()), false)
					]);
					e.color(0x33ddff);

					e
				});
				m
			}).await;

		if let Err(why) = msg {
			println!("Error sending message: {:?}",why);
		}
	}
	Ok(())
}