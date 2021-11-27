use songbird::{ EventHandler as VoiceEventHandler, EventContext, Event };
use serenity::prelude::Context;
use serenity::model::prelude::Message;
use serenity::async_trait;

use std::sync::Arc;


pub struct EndPlaySound {
	ctx: Arc<Context>,
	msg: Arc<Message>,
}

#[async_trait]
impl VoiceEventHandler for EndPlaySound {
	async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
				 leave(&self.ctx, &self.msg).await;
				 None
    }
}

async fn leave(ctx: &Context, msg: &Message) {
	let guild = msg.guild(&ctx.cache).await.unwrap();
	let guild_id = guild.id;
	
	let manager = songbird::get(ctx).await
		.expect("Songbird voice client placed in at initialization").clone();
	
	let has_handler = manager.get(guild_id).is_some();
	
	if has_handler {
		if let Err(e) = manager.remove(guild_id).await {
			println!("Failed: {:?}", e);
		}
		
		println!("Left voice channel");
	} else {
		println!("Not in a voice channel you dummy");
	}
}