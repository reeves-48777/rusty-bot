///! Contains the code for the set command and it's sub commands
/// ## List of subcommands
/// - mute
/// - flood_delay
/// - clear_calls
///

use serenity::framework::standard::{macros::command, Args, CommandResult };
use serenity::model::prelude::Message;
use serenity::prelude::Context;

use crate::ConfigStore;

#[command]
#[description = "change bot settings and `set` them to the new value"]
#[usage = "[clear_calls, mute, flood_delay] true/false or x (for flood_delay in ms)"]
#[example = "clear_calls true"]
#[example = "flood_delay 1000"]
#[min_args(2)]
// NOTE might use sub_commands macro for this (see documentation of serenity::framework::standard::macros::command)
async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
	let setting = args.single::<String>().unwrap();
	let value = args.single::<String>().unwrap();
	match setting.as_str() {
		"mute" => { 
			println!("calling mute command");
			let bot_conf_lock = {
				let data_read = ctx.data.read().await;
				data_read.get::<ConfigStore>().expect("ConfigStore in TypeMap").clone()
			};
			{
				let mut bot_conf = bot_conf_lock.write().await;
				bot_conf.mute(value.parse().unwrap());
			}
		},
		"clear_calls" => {
			println!("calling clear_calls command");
			let bot_conf_lock = {
				let data_read = ctx.data.read().await;
				data_read.get::<ConfigStore>().expect("Config Store in TypeMap").clone()
			};
			{
				let mut bot_conf = bot_conf_lock.write().await;
				bot_conf.clear_calls(value.parse().unwrap());
			}
		},
		"flood_delay" => {
			println!("calling flood_delay command");
			let bot_conf_lock = {
				let data_read = ctx.data.read().await;
				data_read.get::<ConfigStore>().expect("ConfigStore in TypeMap").clone()
			};
			{
				let mut bot_conf = bot_conf_lock.write().await;
				bot_conf.set_flood_delay(value.parse().unwrap());
			}
		},
		_ => {
			msg.channel_id.say(&ctx.http, format!("No setting named '{}' is available", value)).await?;
		}
	}
	Ok(())
}

