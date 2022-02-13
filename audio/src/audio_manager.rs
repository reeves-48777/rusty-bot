use super::{SoundStore,CachedSound, ASSETS_DIR, EndPlaySound};
use songbird::{self, TrackEvent, Songbird, Call, Event, driver::Bitrate, ffmpeg};
use songbird::input::{cached::Compressed, self};

use serenity::prelude::Context;
use serenity::model::prelude::{Message,GuildId,ChannelId};

use tokio::sync::{RwLock, Mutex};


use std::sync::Arc;
use std::{collections::HashMap, fs};

type Queue = HashMap<String, CachedSound>;

struct SoundQueue {
    prev: Queue,
    next: Queue,
}

impl SoundQueue {
    fn new() -> Self {
        Self {
            prev: HashMap::new(),
            next: HashMap::new(),
        }
    }
}


pub struct AudioManager {
    ctx: Arc<Context>,
    msg: Arc<Message>,
    assets: Arc<Vec<String>>,
    guild_id: Option<GuildId>,
    connect_to: Option<ChannelId>,
    handler_lock: Option<Arc<Mutex<Call>>>,
    manager: Option<Arc<Songbird>>,
    queue: SoundQueue,
}

//TODO Rework on this
impl AudioManager {
    pub fn new(ctx: Arc<Context>, msg: Arc<Message>) -> Self {
        let assets: Vec<String> = fs::read_dir(ASSETS_DIR).expect("Assets directory exists").map(|f| f.unwrap().path().to_str().unwrap().to_string()).collect();
        Self {
            ctx,
            msg,
            assets: Arc::new(assets),
            guild_id: None,
            connect_to: None,
            handler_lock: None,
            manager: None,
            queue: SoundQueue::new(),
        }
    }

    /// inits the audio manager struct
    pub async fn init(&mut self) {
        let ctx = self.ctx.clone();
        let msg = self.msg.clone();
        let guild = msg.guild(ctx.cache.clone()).await.unwrap();
        let guild_id = guild.id;
        self.audio_cache_map_lock = Some(self.ctx.data.read().await.get::<SoundStore>().unwrap().clone());

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
        let manager = songbird::get(&self.ctx).await
            .expect("Songbird voice client registered at initialization").clone();
        
        let connect_to = self.connect_to.unwrap();
        
        let (handler_lock, success_reader) = manager
            .join(self.guild_id.unwrap(), connect_to).await;
        
        self.manager = Some(manager);
        self.handler_lock = Some(handler_lock);
        // self.success_reader = Some(success_reader);
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
        let new_asset_name = self.pick_random_asset_name();
        self.fetch_or_create(new_asset_name).await;
        
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
    fn pick_random_asset_name(&mut self) -> String {
        let rand_path = self.assets_paths[rand::random::<usize>() % self.assets_paths.len()].clone();
        println!("Using file: {}", &rand_path);
        String::from(rand_path)
    }

    /// fetch a ressource from the cache map or create a new one then cache it
    async fn fetch_or_create(&self, asset_name: String) -> &CachedSound {
        let sources = self.audio_cache_map_lock.unwrap().read().await;
        match sources.get(&asset_name) {
            Some(source) => {
                todo!();
            },
            None => { 
                todo!();
            }
        }
    }

    // TODO change this
    /// creates a ressource from a path and cache it
    /// We store the value in a temp value
    async fn create_and_cache(&mut self, file_path: String) {
        let source = Compressed::new(
            input::ffmpeg(&file_path).await.expect("File exists"),
            Bitrate::Max
        ).expect("These parameters are well defined");
        let _ = source.raw.spawn_loader();
        let cached = CachedSound::Compressed(source);
        {
            let mut audio_cache_map = self.audio_cache_map_lock.as_ref().unwrap().write().await;
            audio_cache_map.insert(String::from(file_path), cached.clone());
        }
        self.temp_asset = Some(Box::new(cached))
    }
}


fn fetch_random_from_sources(sources: &HashMap<String, CachedSound>) -> &String {
    let keys: Vec<&String> = sources.keys().collect();
    let source_name = keys[rand::random::<usize>() % keys.len()];
    println!("Using file: {}", source_name);
    source_name
}

pub fn init_cache_map(am: AudioManager) -> HashMap<String, CachedSound> {
    let mut cache_map: HashMap<String, CachedSound> = HashMap::new();
    for filename in am.assets {
        let data = Compressed::new(
            input::ffmpeg(&filename).await.expect("File exists"),
            Bitrate::Max,   //todo check the bitrate of the file and enter the same here
        ).expect("These parameters are well defined");
        let _ = source.raw.spawn_loader();
        let cached = CachedSound::Compressed(data);
        cache_map.insert(filename.clone(), cached);
    }
    cache_map
}
