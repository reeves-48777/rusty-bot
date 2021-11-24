use serenity::framework::standard::{CommandResult, Args, macros::{group, command}};
use serenity::utils::MessageBuilder;
use serenity::model::prelude::*;
use serenity::prelude::*;

use super::BOT_COMMANDS;



#[group]
#[commands(mooscles, poomp, flood, clear)]
#[only_in(guilds)]
#[description = "General commands for the bot"]
struct General;

#[command]
#[aliases("m")]
#[description = "basic ping pong command, the bot will write 'COCA COLA ESPUMAAA' in the text channel"]
async fn mooscles(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "COCA COLA ESPUMAAA").await?;
    Ok(())
}

use crate::bot::AudioManager;
#[command]
#[aliases("p")]
#[description = "the bot will connect to the voice channel you are in and will say stuff extracted from the bulk bogan vid"]
async fn poomp(ctx: &Context, msg: &Message) -> CommandResult {
    // remove_command_msg(ctx, msg).await;
    let mut audio_manager = AudioManager::new(ctx, msg);
    audio_manager.init().await;
    audio_manager.join().await;
    audio_manager.play_random_asset().await;
    Ok(()) 
}


use std::num::{IntErrorKind};
#[command]
#[aliases("f")]
#[description = "bot will ping the mentionned user x time"]
#[usage = "@User(s) x"]
#[example = "@User x"]
#[example = "@User1 @User2 x"]
#[min_args(2)]
// NOTE: may check if caching can improve flood (like not 'blocking' after 5 messages) 
async fn flood(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let floods = args.raw().last().unwrap().parse::<i32>();

    match floods {
        Ok(n) => {
            let targets = &msg.mentions;
            
            for _ in 0..n {
                for target in targets {
                    let content = MessageBuilder::new().push(target).build();
                    msg.channel_id.say(&ctx.http, content).await?;
                }
            }
        },

        Err(e) => {
            match e.kind() {
                IntErrorKind::Empty | IntErrorKind::InvalidDigit => {
                    msg.channel_id.say(&ctx.http, "Il faut saisir un nombre apres les mentions pour flood mec").await?;
                },
                _ => {
                    eprintln!("An error occured: {}", e);
                }
            }
        }
    }
    Ok(())
}

use chrono::prelude::*;

#[command]
#[aliases("clr")]
#[description = "Removes bot messages / clean the channel"]
#[usage = "[@User] [n]"]
#[example = " // deletes 50 messages sent by the bot"]
#[example = "10 // deletes 10 messages sent by the bot"]
#[example = "@User // deletes command calls sent by @User"]
#[min_args(0)]
#[max_args(2)]
// TODO optimize this thing
// TODO check whether if there is MANAGE_MESSAGES permission
async fn clear(ctx: &Context, msg: &Message) -> CommandResult {

    // creating a DateTime<Utc> in order to later check the age of the message
    let now = Utc::now();

    println!("calling clear command");

    let bot_id = ctx.cache.current_user().await.id;
    
    // retreiving the messages before the command call
    let messages = msg.channel_id.messages(&ctx.http, |retreiver| {
        retreiver.before(&msg.id).limit(100)
    }).await?;

    
    let to_remove = {
        messages.iter().filter(move |msg| {
            // checks that the messages are less than 14 days old
            let max_old = {
                // awww yiss creating a duration by substracting now and the message timestamp thanks chrono crate devs
                let age = now - msg.timestamp;
                age.num_days() < 14 
            };
            let is_bot = msg.author.id == bot_id;
            let is_command_call = BOT_COMMANDS.iter().any(|cmd| msg.content.starts_with(cmd));

            // the messages to delete have to be less than 14 days old and have to be messages sent by the bot or command calls
            max_old && (is_bot || is_command_call)
        })
    };

    // let last = args.raw().last();
    // let mut _n = 100;
    // if let Some(arg) = last {
    //     if let Ok(limit) = arg.parse::<usize>() {
    //         _n = limit
    //     }
    // }
    
    msg.channel_id.delete_messages(&ctx.http, to_remove).await.unwrap();

    Ok(())
}