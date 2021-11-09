#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

// ---------- IMPORTS ---------- //
use std::
{
    collections::HashMap,
    convert::TryInto,
    env,
    fs,
    sync:: 
    {
        Arc, Weak 
    }
};

use serenity::
{
    FutureExt,
    Result as SerenityResult,
    async_trait,
    client::{ 
        Client, Context, EventHandler 
    },
    framework::
    {
        StandardFramework,
        standard::
        {
            Args,
            CommandResult,
            macros:: { command, group }
        }
    }, 
    model::
    {
        channel::Message,
        gateway::Ready,
        guild, id::
        {
            ChannelId, GuildId
        }, 
        misc::Mentionable
    }, 
    prelude::*
};

use songbird::
{
    Call, 
    Event, 
    EventContext, 
    EventHandler as VoiceEventHandler, 
    SerenityInit, 
    Songbird, 
    TrackEvent, 
    driver::Bitrate, 
    error::JoinError, 
    input::
    {
        self,
        cached:: { Compressed, Memory },
        Input
    }
};


pub mod bot;


// -------- CONSTS ---------- //
const ASSETS_DIR: &str ="assets";

// --------- STRUCTS --------- //
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}

#[group]
#[commands(mooscles, poomp)]
struct General;

// ---------- MAIN FUNCTION ----------- //
#[tokio::main]
async fn main() {

    //tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_BOT_TOKEN").expect("token env var not set");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("$"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;

        let audio_map = init_assets_in_cache().await;
        
        data.insert::<SoundStore>(Arc::new(Mutex::new(audio_map)));
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

use bot::audio::*;
#[command]
#[only_in(guilds)]
async fn poomp(ctx: &Context, msg: &Message, _args:Args) -> CommandResult {
    let mut audio_manager = AudioManager::new(ctx, msg);
    audio_manager.init().await;
    audio_manager.join().await;
    audio_manager.play_random_asset().await;
    Ok(()) 
}



fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
