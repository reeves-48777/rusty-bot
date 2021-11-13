use serenity::framework::standard::{ macros::{group,command}, Args, CommandResult };
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::bot::config::*;


#[group]
// Sets the prefixes config and conf for this group
// allows us to call commands that will configure / change the behaviour of the bot
// via `$config cmd` or `$conf cmd` instead of just `$`
#[prefixes("config", "conf")]
#[description = "Group with commands that allows to configure the bot"]
#[summary = "Config stuff"]
#[commands(set,show)]
struct Config;


#[command]
async fn config_help(_ctx: &Context, _msg: &Message) -> CommandResult {
	todo!()
}


#[command]
#[aliases("s")]
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

#[command]
async fn show(ctx: &Context, msg: &Message) -> CommandResult {
	println!("calling show_config command");
	let bot_conf_lock = {
		let data_read = ctx.data.read().await;
		data_read.get::<ConfigStore>().unwrap().clone()
	};
	{
		let bot_conf = bot_conf_lock.read().await;
		// TODO use embed instead (see CreateMessageBuilder example in serenity repo)
		msg.channel_id.say(&ctx.http, format!("```\nBot configuration\n\nclear_command_calls: {}\nbot_muted: {}\nflood_delay: {}\n```",bot_conf.get_clear_calls(), bot_conf.muted(), bot_conf.get_flood_delay())).await?;
	}
	Ok(())
}