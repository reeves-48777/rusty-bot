use serenity::framework::standard::{CommandResult,macros::command};
use serenity::prelude::Context;
use serenity::model::prelude::Message;

#[command]
#[aliases("m")]
#[description = "basic ping pong command, the bot will write 'COCA COLA ESPUMAAA' in the text channel"]
async fn mooscles(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "COCA COLA ESPUMAAA").await?;
    Ok(())
}