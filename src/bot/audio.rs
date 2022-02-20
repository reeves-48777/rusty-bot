use std::{
	fs, fs::DirEntry,
	sync::Arc,
	io, collections::HashSet
};

use serenity::{
	async_trait, 
	client::Context,
		model::{
			channel::Message,
			id::{ChannelId, GuildId}
		},
		prelude::{TypeMapKey, RwLock, Mutex}
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
	},
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

impl From<CachedSound> for Input {
	fn from(obj: CachedSound) -> Self {
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

type SoundQueue = Vec<CachedSound>;

// struct SoundQueue {
// 	sound: Option<CachedSound>,
// }

// impl SoundQueue {
// 	fn new() -> Self {
// 		Self {
// 			sound: None,
// 		}
// 	}
// }

// impl Iterator for SoundQueue {
// 	type Item = CachedSound;

// 	fn next(&mut self) -> 
// }
fn gen_index(len: usize) -> usize {
	rand::random::<usize>() % len
}


// on récupere les son depuis le dossier assets
// on les met en cache dans une hashmap avec le nom du fichier en clé et son binaire en valeur
pub async fn init_assets_in_cache() -> SoundQueue {
	let dir = fs::read_dir(ASSETS_DIR).expect("assets directory in projet");
	let dir: Vec<io::Result<DirEntry>> = dir.collect();
	let dir_len = dir.len();

	// on init les queues
	let mut cache_map = Vec::with_capacity(dir_len);
	let mut indexes = HashSet::<usize>::with_capacity(dir_len);
	
	// generation de la suite de nombre aléatoires uniques
	for _i in 0..dir_len {
		let mut uniq = false;
		while !uniq {
			let index = gen_index(dir_len);
			if indexes.insert(index) {
				println!("using index: {}", index);
				uniq = true;
			}
		}
	}
	// génération des files dans un ordre aléatoire
	for index in indexes {
		let path = String::from(dir[index].as_ref().unwrap().path().to_str().unwrap());
		println!("adding file: {} to the cache map", &path);
		let data = Compressed::new(
			input::ffmpeg(&path).await.expect("File not found"),
			Bitrate::Auto
		).expect("These parameters are well defined");
		let _ = data.raw.spawn_loader();
		cache_map.push(CachedSound::Compressed(data));
	}
    cache_map
}

/// structure utilisée pour gérer l'action à effectuer à la fin des sons joués
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

// on quitte le channel dans lequel on est
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
	type Value = Arc<RwLock<SoundQueue>>;
}

pub struct AudioManager<'a> {
	ctx: &'a Context,
	msg: &'a Message,
	guild_id: Option<GuildId>,
	connect_to: Option<ChannelId>,
	handler_lock: Option<Arc<Mutex<Call>>>,
	manager: Option<Arc<Songbird>>,
	success_reader: Option<Result<(), JoinError>>,
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
			success_reader: None,
		}
	}
	pub async fn init<'a>(&mut self) {
		let ctx = self.ctx;
		let msg = self.msg;
		let guild = msg.guild(ctx.cache.clone()).await.unwrap();
		let guild_id = guild.id;

		// on recupere l'id du channel sur lequel l'utilisateur qui apelle la commande est
		let channel_id = guild
			.voice_states.get(&msg.author.id)
			.and_then(|voice_state| voice_state.channel_id);

		// si l'id existe (l'utilisateur est dans un channel), on s'y connecte
		match channel_id {
			Some(channel) => {
				self.connect_to = Some(channel)
			},
			// sinon le bot renvoie un message d'erreur / d'explication
			None => {
				msg.channel_id.say(&ctx.http, "You need to be in a voice channel to call this command (for now)").await.unwrap();
			}
		}

		// on update le guild-id
		self.guild_id = Some(guild_id);
	}

	/// rejoindre un channel puis on initialise:
	/// - les valeurs du lock (thread safety)
	/// - le manager de songbird (pour jouer des fichiers audio)
	/// - le success_reader (valeur de callback si le bot a pu se connecter/rejoindre le channel)
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

	/// On joue un son random depuis les sources
	pub async fn play_random_asset(&self) {
		let handler_lock = self.handler_lock.as_ref().unwrap();
		// on lock pour éviter de modifier les données en meme temps qu'on les lit
		let mut handler = handler_lock.lock().await;

		println!("Joined Voice Chan: {}", self.connect_to.unwrap().name(&self.ctx.cache).await.unwrap());

		let queue_lock = {
			let data_read = self.ctx.data.read().await;
			data_read.get::<SoundStore>().expect("ConfigStore in TypeMap").clone()
		};

		#[allow(unused_assignments)]
		let mut source = None;
		{
			let mut queue = queue_lock.write().await;
			source = queue.pop();
		}

		let sound = handler.play_source(source.unwrap().into());
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


// si l'audio Manager jouer un son 						=> etat = playing
// si l'audio manager attend un nouveau call de poomp 	=> etat = pending/waiting
// si l'audio manager ne fait rien 						=> etat = idle