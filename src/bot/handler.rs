use serenity::async_trait;
use serenity::client::{EventHandler, Context};
use serenity::model::prelude::{Message, Ready};

use super::ConfigStore;
use crate::commands::BOT_COMMANDS;


pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // find a way with hooks or maybe before and after
    async fn message(&self, ctx: Context, new_message: Message) {
        let msgs_config = &ctx.data.read().await.get::<ConfigStore>().unwrap().clone();
        let mut cmd = false;
        
        for command_name in BOT_COMMANDS.iter() {
            if new_message.content.starts_with(command_name) {
                cmd = true
            }
        }
        
        if cmd && msgs_config.read().await.get_clear_calls() {
            new_message.delete(ctx.http).await.unwrap();
        } else {
            println!("[{}] <{}> {}", new_message.timestamp, new_message.author.name, new_message.content);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Logged in as {}", ready.user.name);
    }
}