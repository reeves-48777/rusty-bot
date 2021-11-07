use serenity::{async_trait, client::{Context, EventHandler}};
use serenity::model::gateway::Ready;

pub mod audio;

pub use audio::AudioManager;




pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is connected", ready.user.name);
	}
}