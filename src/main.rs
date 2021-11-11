// ---------- IMPORTS ---------- //
use std::{env, sync::Arc};
use songbird::SerenityInit;

use serenity::
{
    Result as SerenityResult,
    async_trait,
    client::{ 
        Client,
        Context,
        EventHandler 
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
    model::{channel::Message, gateway::Ready,}, 
    prelude::*,
    utils::MessageBuilder,
};

pub mod bot;

use bot::Configuration;

struct ConfigStore;

impl TypeMapKey for ConfigStore {
    type Value = Arc<Mutex<Configuration>>;
}


// --------- STRUCTS --------- //

struct CommandNameStore;

impl TypeMapKey for CommandNameStore {
    type Value = Arc<Mutex<Vec<String>>>;
}


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }

    // TODO: filter the content of the message in order to check if the message can be deleted before calling a command or
    // or if we have to pass message data in the called command (e.g: flood command needs a reference to the create message)
    // because it contains the flood number and the targeted users so we can't delete the message before passing the reference
    // TODO: Might rework on the command system and check wether the created message is a command instead of checking if the message contains a command name
    // NOTE: there are once again multiple ways to solve this, the easiest one is to remove the message after the commands
    // on the other hand it will feel weird on usage (might have to ask users for some feedback on this)
    // we could also not delete command after calling them, but in this case making a channel especially for the bot could be necessary, because it will mess general channels
    // Another way to solve this could be to just not reply to the message and write a message on the channel by mentioning the user
    async fn message(&self, ctx: Context, new_message: Message) {
        let msgs_config = &ctx.data.read().await.get::<ConfigStore>().cloned().unwrap();
        let data = &ctx.data.read().await.get::<CommandNameStore>().cloned().unwrap();
        let command_names = data.lock().await;
        let mut contains_command = false;

        
        for command_name in command_names.iter() {
            if new_message.content.contains(command_name) {
                contains_command = true
            }
        }
        
        if contains_command {
            if msgs_config.lock().await.get_config() {
                new_message.delete(ctx.http).await.unwrap();
            }
        }
        
    }
}

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
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;

        // NOTE: reworking on the audio map might be necessary.
        // Here we are caching all the sounds from the assets directory at start, but the program could not use all of them due to the `pick_random_source()` function
        // so caching all the sounds kinda waste memory. There are two ways to solve this
        // 1 - Just implement a better random that will play all the sounds but in a random order (like a playlist on spotify for example) or
        // 2 - Pick a random sound from the assets, play it and cache it, and implement a better random (same files can be played twice in a row)
        let audio_map = init_assets_in_cache().await;
        let mut command_config = Configuration::new(false);
        let command_names = vec![
            String::from("$mooscles"),
            String::from("$poomp"),
            String::from("$flood")];
        
        data.insert::<SoundStore>(Arc::new(Mutex::new(audio_map)));
        data.insert::<ConfigStore>(Arc::new(Mutex::new(command_config)));
        data.insert::<CommandNameStore>(Arc::new(Mutex::new(command_names)));
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
async fn clear(ctx: &Context, msg: &Message) -> CommandResult{
    Ok(())
}

// #[command]
// #[only_in(guilds)]
// async fn config(ctx: &Context, msg: &Message) -> CommandResult {
//     let mut data = &mut ctx.data.read().await.get::<ConfigStore>().unwrap();
//     let command = String::from(msg.content.clone());
//     let mut set = false;

//     if command.contains("set") {
//         set = true;
//     }

//     if command.contains("message_remove") {
//         if set {
//             if command.contains("true") {
//                 data.lock().await.set_config(true);
//             }
//         }
//     }
//     Ok(())
// }

fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}