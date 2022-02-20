use serenity::framework::standard::{ macros::{group,command}, Args, CommandResult };
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::bot::config::*;


#[group]
// Sets the prefixes config and conf for this group
// allows us to call commands that will configure / change the behaviour of the bot
// via `$config cmd` or `$conf cmd` instead of just `$`
#[prefixes("config", "conf")]
#[description = "Commands that allows you to change or see the bot's current config"]
#[commands(set,show)]
struct Config;


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
		"wait_delay" => {
			println!("calling wait_delay comamand");
			let bot_conf_lock = {
				let data_read = ctx.data.read().await;
				data_read.get::<ConfigStore>().expect("ConfigStore in TypeMap").clone()
			};
			{
				let mut bot_conf = bot_conf_lock.write().await;
				bot_conf.set_poomp_delay(value.parse().unwrap());
			}

		},
		_ => {
			msg.channel_id.say(&ctx.http, format!("No setting named '{}' is available", value)).await?;
		}
	}
	Ok(())
}

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
						("flood_delay", format!("{} seconds", bot_conf.get_flood_delay()), false),
						("poomp_delay", format!("{} seconds", bot_conf.get_poomp_delay()), false),
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