use serenity::framework::standard::{CommandResult, Args, macros::{group, command}};
use serenity::utils::MessageBuilder;
use serenity::model::prelude::*;
use serenity::prelude::*;



#[group]
#[commands(mooscles, poomp, flood)]
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
#[aliases("f")]
#[description = "bot will ping the mentionned user x time"]
#[usage = "@User(s) x"]
#[example = "@User x"]
#[example = "@User1 @User2 x"]
#[min_args(2)]
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
// TODO
// this command has to clear all messages sent by the bot or the commands sent by the mentionned user 
async fn clear(_ctx: &Context, _msg: &Message) -> CommandResult{
    todo!();
}