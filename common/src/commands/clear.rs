use serenity::framework::standard::{macros::command, CommandResult};
use serenity::prelude::Context;
use serenity::model::prelude::Message;

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
            let is_command_call = super::BOT_COMMANDS.iter().any(|cmd| msg.content.starts_with(cmd));

            // the messages to delete have to be less than 14 days old and have to be messages sent by the bot or command calls
            max_old && (is_bot || is_command_call)
        })
    };
    msg.channel_id.delete_messages(&ctx.http, to_remove).await.unwrap();

    Ok(())
}