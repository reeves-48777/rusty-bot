use serenity::async_trait;
use serenity::client::{EventHandler, Context};
use serenity::model::prelude::{Message, Ready};

use config::ConfigStore;
use common::commands::BOT_COMMANDS;


pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_msg: Message) {
        let config_lock = &ctx.data.read().await.get::<ConfigStore>().unwrap().clone();
        let mut cmd = false;

        for cmd_name in BOT_COMMANDS.iter() {
            if new_msg.content.starts_with(cmd_name) {
                cmd = true
            }
        }

        if cmd && config_lock.read().await.get_clear_calls() {
            new_msg.delete(ctx.http).await.unwrap();
        }
    }

    async fn ready(&self, _:Context, ready: Ready) {
        println!("Logged in as {}", ready.user.name);
    }
}