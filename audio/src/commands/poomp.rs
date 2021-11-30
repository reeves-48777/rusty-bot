use serenity::framework::standard::{macros::command, CommandResult};
use serenity::prelude::Context;
use serenity::model::prelude::Message;


// TODO fix
use crate::AudioManager;
#[command]
#[aliases("p")]
#[description = "the bot will connect to the voice channel you are in and will say stuff extracted from the bulk bogan vid"]
async fn poomp(ctx: &Context, msg: &Message) -> CommandResult {
    let mut audio_manager = AudioManager::new(ctx, msg);
    audio_manager.init().await;
    audio_manager.join().await;
    audio_manager.play_random_asset().await;
    Ok(()) 
}