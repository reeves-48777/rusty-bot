use std::{
	collections::HashMap,
	sync::Arc,
	fs
};

use serenity::{
	async_trait, 
	client::{
		Context, EventHandler},
		model::{
			channel::{self, Message},
			id::{ChannelId, GuildId}
		},
		prelude::{Mentionable, TypeMapKey, Mutex}
};

use songbird::{
	Call,
	Event,
	EventContext,
	EventHandler as VoiceEventHandler,
	SerenityInit,
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
enum AudioManagerDynComponent {
	Handler(Arc<Mutex<Call>>),
	Connection(ChannelId),
	JoinResult(Result<(), JoinError>),
	SoundManager(Arc<Songbird>)
}

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

pub async fn init_files_in_cache() -> HashMap<String, CachedSound> {
	let mut cache_map = HashMap::new();
    for file in fs::read_dir("assets").expect("assets directory not found") {
        let path = String::from(file.unwrap().path().to_str().unwrap());
        let source = Compressed::new(
            input::ffmpeg(&path).await.expect("File not found"),
            Bitrate::Max
        ).expect("These parameters are well defined");
        let _ = source.raw.spawn_loader();
        cache_map.insert(path.split('\\').last().unwrap().into(), CachedSound::Compressed(source));
    }
    cache_map
}


struct EndPlaySound {
	ctx: Context,
	msg: Message
}

#[async_trait]
impl VoiceEventHandler for EndPlaySound {
	async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
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

struct SoundStore;

impl TypeMapKey for SoundStore {
	type Value = Arc<Mutex<HashMap<String, CachedSound>>>;
}

pub struct AudioManager<'a> {
	guild_id: GuildId,
	ctx: &'a Context,
	msg: &'a Message,
	dyn_components: HashMap<String, AudioManagerDynComponent>,
	connect_to: Option<ChannelId>,
	sound_manager: Option<Arc<Songbird>>,
}

impl AudioManager<'_> {
	pub async fn init<'a>(ctx: &'a Context, msg: &'a Message) -> AudioManager {
		let guild = msg.guild(&ctx.cache).await.unwrap();
		let guild_id = guild.id;

		let mut audio_manager = AudioManager {
			guild_id,
			ctx,
			msg,
			dyn_components: HashMap::new(),
			connect_to: None,
			sound_manager: None,
		};

		let channel_id = guild
			.voice_states.get(&msg.author.id)
			.and_then(|voice_state| voice_state.channel_id);

		if let Some(channel) = channel_id {
			audio_manager.connect_to = Some(channel);
		}

		audio_manager
	}

	pub async fn join(&mut self) {
		let manager = songbird::get(self.ctx).await
			.expect("Songbird voice client registered at initialization").clone();

		let connect_to = self.connect_to.unwrap();

		let (handler_lock, success_reader) = manager
			.join(self.guild_id, connect_to).await;

		self.sound_manager = Some(manager);	
		self.dyn_components.insert(String::from("handler_lock"), AudioManagerDynComponent::Handler(handler_lock));
		self.dyn_components.insert(String::from("success_reader"), AudioManagerDynComponent::JoinResult(success_reader));
	}

	pub async fn play_random_asset(&self) {
		let mut handler_lock = None;
		if let AudioManagerDynComponent::Handler(hand_lock) = self.dyn_components.get("handler_lock").unwrap() {
			handler_lock = Some(hand_lock);
		}
		let handler_lock = handler_lock.unwrap();

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
				msg: self.msg.clone()
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