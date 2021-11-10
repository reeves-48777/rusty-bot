use std::{collections::HashMap , fs, sync::Arc};

use serenity::{
	async_trait, 
	client::Context,
		model::{
			channel::Message,
			id::{ChannelId, GuildId}
		},
		prelude::{Mentionable, TypeMapKey, Mutex}
};

use songbird::{
	Call,
	Event,
	EventContext,
	EventHandler as VoiceEventHandler,
	Songbird,
	TrackEvent,
	driver::Bitrate,
	error::JoinError,
	input::{
		self,
		Input,
		cached::{Compressed, Memory}
	}
};


const ASSETS_DIR: &str = "assets";

#[derive(Debug)]
pub enum CachedSound {
	Compressed(Compressed),
	Uncompressed(Memory),
}

impl From<&CachedSound> for Input {
	fn from(obj: &CachedSound) -> Self {
		use CachedSound::*;
		match obj {
			Compressed(comp) => comp.new_handle()
				.into(),
			Uncompressed(uncomp) => uncomp.new_handle()
				.try_into()
				.expect("Failed to create decode for memory source")
		}
	}
}

pub async fn init_assets_in_cache() -> HashMap<String, CachedSound> {
	let mut cache_map = HashMap::new();
    for file in fs::read_dir(ASSETS_DIR).expect("assets directory not found") {
        let path = String::from(file.unwrap().path().to_str().unwrap());
        let source = Compressed::new(
            input::ffmpeg(&path).await.expect("File not found"),
            Bitrate::Max
        ).expect("These parameters are well defined");
        let _ = source.raw.spawn_loader();
        // cache_map.insert(path.split('\\').last().unwrap().into(), CachedSound::Compressed(source));
		cache_map.insert(path.into(), CachedSound::Compressed(source));
    }
    cache_map
}

struct EndPlaySound {
	ctx: Context,
	msg: Message,
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

pub struct SoundStore;

impl TypeMapKey for SoundStore {
	type Value = Arc<Mutex<HashMap<String, CachedSound>>>;
}

pub struct AudioManager<'a> {
	ctx: &'a Context,
	msg: &'a Message,
	guild_id: Option<GuildId>,
	connect_to: Option<ChannelId>,
	handler_lock: Option<Arc<Mutex<Call>>>,
	manager: Option<Arc<Songbird>>,
	success_reader: Option<Result<(), JoinError>>
}

impl AudioManager<'_> {
	pub fn new<'a>(ctx: &'a Context, msg: &'a Message) -> AudioManager<'a> {
		AudioManager {
			ctx,
			msg,
			guild_id: None,
			connect_to: None,
			handler_lock: None,
			manager: None,
			success_reader: None
		}
	}
	pub async fn init<'a>(&mut self) {
		let ctx = self.ctx;
		let msg = self.msg;
		let guild = msg.guild(ctx.cache.clone()).await.unwrap();
		let guild_id = guild.id;

		let channel_id = guild
			.voice_states.get(&msg.author.id)
			.and_then(|voice_state| voice_state.channel_id);

		if let Some(channel) = channel_id {
			self.connect_to = Some(channel)
		}
		self.guild_id = Some(guild_id);
	}

	pub async fn join(&mut self) {
		let manager = songbird::get(self.ctx).await
			.expect("Songbird voice client registered at initialization").clone();

		let connect_to = self.connect_to.unwrap();

		let (handler_lock, success_reader) = manager
			.join(self.guild_id.unwrap(), connect_to).await;

		self.manager = Some(manager);
		self.handler_lock = Some(handler_lock);
		self.success_reader = Some(success_reader);
	}

	pub async fn play_random_asset(&self) {
		let handler_lock = self.handler_lock.as_ref().unwrap();
		let mut handler = handler_lock.lock().await;

		println!("Joined Voice Chan: {}", self.connect_to.unwrap().mention());

		let sources_lock = self.ctx.data.read().await.get::<SoundStore>().cloned().expect("Sound cache was initialized at startup");
		let sources_lock_for_evt = sources_lock.clone();
		let sources = sources_lock.lock().await;
		let source = sources.get(fetch_random_from_sources(&sources)).expect("handle placed into cache at startup");

		let sound = handler.play_source(source.into());
		let _ = sound.set_volume(1.0);

		let _ = sound.add_event(
			Event::Track(TrackEvent::End),
			EndPlaySound {
				ctx: self.ctx.clone(),
				msg: self.msg.clone(),
			},	 
		);
	}
}
fn fetch_random_from_sources(sources: &HashMap<String, CachedSound>) -> &String {
	let keys: Vec<&String> = sources.keys().collect();
	let source_name = keys[rand::random::<usize>() % keys.len()];
	println!("Using file: {}", source_name);
	source_name
}