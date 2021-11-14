// ---------- IMPORTS ---------- //
use std::{collections::HashSet, env, sync::Arc};
use songbird::SerenityInit;

use serenity::
{Result as SerenityResult, client::{ 
        Client,
        Context,
    }, framework::
    {StandardFramework, standard::
        {Args, CommandGroup, CommandResult, HelpOptions, help_commands, macros::{command, group}}}, http::Http, model::{channel::Message, id::UserId}, prelude::*, utils::MessageBuilder};

use serenity::framework::standard::macros::{help, hook};

mod bot;
mod hooks;
mod commands;

use bot::*;
use commands::config::CONFIG_GROUP;
use commands::help::MY_HELP;
use commands::general::GENERAL_GROUP;

use bot::audio::{SoundStore, init_assets_in_cache};


#[hook]
async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}


// ---------- MAIN FUNCTION ----------- //
#[tokio::main]
async fn main() {
    //tracing_subscriber::fmt::init();
    let token = env::var("DISCORD_BOT_TOKEN").expect("token env var not set");

    let http = Http::new_with_token(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}",why),
            }
        },
        Err(why) => panic!("Could not get application info: {:?}",why)
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                    .prefix("$")
                    .owners(owners))
        .unrecognised_command(unknown_command)
        .help(&MY_HELP)
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

fn _check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}