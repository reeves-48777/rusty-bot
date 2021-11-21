use std::{collections::HashMap, fs::{self, DirEntry}};
use tokio::sync::{RwLock,Mutex};
use std::sync::Arc;

use serenity::{
	async_trait, 
	client::Context,
	model::{
		channel::Message,
		id::{ChannelId, GuildId}
	},
	prelude::{TypeMapKey}
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
#[derive(Clone)]
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



struct EndPlaySound {
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

pub struct SoundStore;

impl TypeMapKey for SoundStore {
	type Value = Arc<RwLock<HashMap<String, CachedSound>>>;
}

// NOTE trying some stuff here
pub struct AudioManager {
    ctx: Arc<Context>,
    msg: Arc<Message>,
    assets_paths: Arc<Vec<String>>,
    audio_cache_map_lock: Arc<RwLock<HashMap<String, CachedSound>>>,
    guild_id: Option<GuildId>,
    connect_to: Option<ChannelId>,
    handler_lock: Option<Arc<Mutex<Call>>>,
    manager: Option<Arc<Songbird>>,
    success_reader: Option<Result<(), JoinError>>
}

impl AudioManager {
    pub fn new(ctx: Context, msg: Message) -> AudioManager {
        let assets: Vec<String> = fs::read_dir(ASSETS_DIR).expect("Assets directory exists").map(|f| f.unwrap().path().to_str().unwrap().to_string()).collect();
        AudioManager {
            ctx: Arc::new(ctx),
            msg: Arc::new(msg),
            assets_paths: Arc::new(assets),
            audio_cache_map_lock: Arc::try_new_uninit().unwrap(),
            guild_id: None,
            connect_to: None,
            handler_lock: None,
            manager: None,
            success_reader: None
        }
    }

    /// inits the audio manager struct
    pub async fn init(&mut self) {
        let ctx = self.ctx;
        let msg = self.msg;
        let guild = msg.guild(ctx.cache.clone()).await.unwrap();
        let guild_id = guild.id;
        self.audio_cache_map_lock = self.ctx.data.read().await.get::<SoundStore>().unwrap();

        let channel_id = guild
            .voice_states.get(&msg.author.id)
            .and_then(|voice_state| voice_state.channel_id);

        match channel_id {
            Some(channel) => {
                self.connect_to = Some(channel)
            },
            None => {
                msg.channel_id.say(&ctx.http, "You need to be in a voice channel to call this command (for now)").await.unwrap();
            }
        }

        if let Some(channel) = channel_id {
            self.connect_to = Some(channel)
        }
        self.guild_id = Some(guild_id);
    }

    /// Creates / inits a connection to the voice channel the user is connected to
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

    /// Choose a random asset from the CacheMap and play it
    pub async fn play_random_asset(&self) {
        let handler_lock = self.handler_lock.as_ref().unwrap();
        let mut handler = handler_lock.lock().await;
        
        match self.connect_to {
            Some(connection) => {
                println!("Joined voice channel: {}", connection.name(&self.ctx.cache).await.unwrap());
            },
            None => {
                println!("User not connected to a voice channel");
            }
        }
        let sources_lock = {
            let data_read = self.ctx.data.read().await;
            data_read.get::<SoundStore>().expect("Audio cache map initialized at startup").clone()
        };
        // let sources_lock_for_evt = sources_lock.clone();
        // let sources_lock = self.ctx.data.read().await.get::<SoundStore>().cloned().expect("Sound cache was initialized at startup");
        let source = self.pick_random_asset(sources_lock).await;
        
        let sound = handler.play_source(source.into());
        let _ = sound.set_volume(1.0);
        
        let _ = sound.add_event(
            Event::Track(TrackEvent::End),
            EndPlaySound {
                ctx: self.ctx.clone(),
                msg: self.msg.clone(),
        });
    }

    /// picks a random asset from the assets directory
    async fn pick_random_asset(&mut self, audio_cache_map_lock: Arc<RwLock<HashMap<String, CachedSound>>>) -> &CachedSound {
        let rand_path = self.assets_paths[rand::random::<usize>() % self.assets_paths.len()].clone();
        println!("Using file: {}", &rand_path);

        let sources = {
            audio_cache_map_lock.read().await
        };
        
        let fetched = sources.get(&rand_path);

        let source = match fetched {
            Some(s) => s,
            None => &create_asset_cache_it_and_return(self.audio_cache_map_lock.clone(), rand_path).await,
        };
        source
    }
}

/// creates an asset from a file_path and cache it into the assets_map
pub async fn create_asset_cache_it_and_return(audio_cache_map_lock: Arc<RwLock<HashMap<String, CachedSound>>>, file_path: String) -> CachedSound {
    let source = Compressed::new(
        input::ffmpeg(&file_path).await.expect("File exists"),
        Bitrate::Max
    ).expect("These parameters are well defined");
    let _ = source.raw.spawn_loader();
    let cached = CachedSound::Compressed(source);
    {
        let mut audio_cache_map = audio_cache_map_lock.write().await;
        audio_cache_map.insert(String::from(file_path), cached.clone());
    }
    cached
}




fn fetch_random_from_sources(sources: &HashMap<String, CachedSound>) -> &String {
    let keys: Vec<&String> = sources.keys().collect();
    let source_name = keys[rand::random::<usize>() % keys.len()];
    println!("Using file: {}", source_name);
    source_name
}

pub fn init_cache_map() -> HashMap<String, CachedSound> {
    let cache_map: HashMap<String, CachedSound> = HashMap::new();
    cache_map
}
