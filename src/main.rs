// ---------- IMPORTS ---------- //
use std::{env, sync::Arc};
use songbird::SerenityInit;

use serenity::
{
    Result as SerenityResult,
    client::{ 
        Client,
        Context,
    },
    framework::
    {
        StandardFramework,
        standard::
        {
            Args,
            CommandResult,
            macros::{command, group}
        }
    }, 
    model::{channel::Message}, 
    prelude::*,
    utils::MessageBuilder,
};

mod bot;
mod commands;

use bot::*;
use commands::config::CONFIG_GROUP;


#[group]
#[commands(mooscles, poomp, flood)]
struct General;

// ---------- MAIN FUNCTION ----------- //
#[tokio::main]
async fn main() {
    //tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_BOT_TOKEN").expect("token env var not set");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$"))
        .group(&GENERAL_GROUP)
        .group(&CONFIG_GROUP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;

        let audio_map = init_assets_in_cache().await;
        let bot_config = ConfigBuilder::new().build();

        data.insert::<SoundStore>(Arc::new(Mutex::new(audio_map)));
        data.insert::<ConfigStore>(Arc::new(RwLock::new(bot_config)));
    }

    tokio::spawn(async move {
        let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
    });

    tokio::signal::ctrl_c().await.unwrap();
    println!("Ctrl-C received, shutting down");
}

// ------------ FUNCTIONS ----------- //

#[command]
async fn mooscles(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "COCA COLA ESPUMAAA").await?;
    Ok(())
}

use bot::{audio::*};
#[command]
#[only_in(guilds)]
async fn poomp(ctx: &Context, msg: &Message, _args:Args) -> CommandResult {
    // remove_command_msg(ctx, msg).await;
    let mut audio_manager = AudioManager::new(ctx, msg);
    audio_manager.init().await;
    audio_manager.join().await;
    audio_manager.play_random_asset().await;
    Ok(()) 
}


use std::num::{ParseIntError, IntErrorKind};
#[command]
#[only_in(guilds)]
// NOTE: may check if caching can improve flood (like not 'blocking' after 5 messages) 
async fn flood(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let floods: Result<i32, ParseIntError> = args.raw().last().unwrap().parse();

    match floods {
        Err(e) => {
            match e.kind() {
                IntErrorKind::Empty | IntErrorKind::InvalidDigit => {
                    msg.channel_id.say(&ctx.http, "Il faut saisir un nombre apres les mentions pour flood espece de fils de truite").await.unwrap();
                },
                _ => {
                    eprintln!("An error occured: {}", e);
                } 
            }
        },
        Ok(n) => {
            let floods = n;
            let targets = &msg.mentions;

            for _ in 0..floods {
                for target in targets.iter() {
                    let content = MessageBuilder::new().push(target).build();
                    msg.channel_id.say(&ctx.http, content).await.unwrap();
                }
            }
        }
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
// TODO
// this command has to clear all messages sent by the bot or the commands sent by the mentionned user 
async fn clear(_ctx: &Context, _msg: &Message) -> CommandResult{
    todo!();
}

fn _check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}