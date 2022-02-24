use std::{
	fs, fs::DirEntry,
	sync::Arc,
	io, collections::HashSet,
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


pub const ASSETS_DIR: &str = "assets";

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

type SoundBuffer = Vec<CachedSound>;

pub struct SoundCache<'lifea> {
	front_buffer: Option<&'lifea SoundBuffer>,
	back_buffer: SoundBuffer,
	directory: Vec<io::Result<DirEntry>>,
	directory_size: usize,
}

impl <'lifea> SoundCache<'lifea> {
	pub fn new() -> Self {
		Self {
			front_buffer: None,
			back_buffer: Vec::new(),
			directory: Vec::new(),
			directory_size: 0,
		}
	}
	pub async fn init(&mut self, dir_path: String) {
		let (dir,dir_len) = get_dir_len(dir_path);
		self.front_buffer = gen_buffer(dir_len, &dir).await;
		self.back_buffer = gen_buffer(dir_len, &dir).await;
	}

	pub async fn read(&mut self) -> Option<CachedSound> {
		if self.front_buffer.len() > 0 {
			if self.back_buffer.len() == 0 {
				println!("Back buffer is empty regenerating ...");
				self.back_buffer= gen_buffer(self.directory_size, &self.directory).await;
			}
			Some(self.front_buffer.pop().unwrap())
		} else {
			println!("Front buffer is empty regenerating ...");
			// self.buffers.0 = gen_buffer(self.directory_size, &self.directory).await;
			// Some(self.buffers.1.pop().unwrap()
			self.front_buffer = self.front_buffer;
			self.back_buffer = gen_buffer(self.directory_size, &self.directory).await;
			Some(self.front_buffer.pop().unwrap())
		}
	}

}

fn get_dir_len(directory_path: String) -> (Vec<io::Result<DirEntry>>, usize) {
	println!("Getting the size of the directory named: {}...", directory_path);
	let dir = fs::read_dir(directory_path).expect("Assets directory specified or default 'assets' directory present in the project");
	let dir: Vec<io::Result<DirEntry>> = dir.collect();
	let dir_len = dir.len();
	println!("DirSize = {}", &dir_len);
	return (dir, dir_len)
}


fn gen_random_order(range: usize) -> HashSet<usize> {
	println!("Generating random order ...");
	let mut indexes: HashSet<usize> = HashSet::with_capacity(range);
	for _ in 0..range {
		let mut uniq = false;
		while !uniq {
			let index = rand::random::<usize>() % range;
			if indexes.insert(index) {
				uniq = true;
			}
		}
	}
	indexes
}

async fn gen_buffer(len: usize, dir: &Vec<io::Result<DirEntry>>) -> SoundBuffer {
	println!("Generating sound buffer ...");
	let mut queue = Vec::with_capacity(len);
	let indexes = gen_random_order(len);
	
	for index in indexes {
		let file_path = String::from(dir[index].as_ref().unwrap().path().to_str().unwrap());
			println!("adding file: {} to the cache", file_path.split('/').last().unwrap());
			// there should be only mp3 audio files in the directory so we build compressed files
			let data = Compressed::new(
				input::ffmpeg(&file_path).await.expect("File in assets directory"),
				Bitrate::Auto
			).expect("These parameters are well defined");

			let _ = data.raw.spawn_loader();
			queue.push(CachedSound::Compressed(data));
	}
	queue
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
	type Value = Arc<RwLock<SoundCache>>;
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

		let cache_lock = {
			let data_read = self.ctx.data.read().await;
			data_read.get::<SoundStore>().expect("ConfigStore in TypeMap").clone()
		};

		#[allow(unused_assignments)]
		let mut source = None;
		{
			let mut cache = cache_lock.write().await;
			source = cache.read().await;
		}
		
		match source {
			Some(data) => {
				let sound = handler.play_source(data.into());
				let _ = sound.set_volume(1.0);
		
				let _ = sound.add_event(
					Event::Track(TrackEvent::End),
					EndPlaySound {
						ctx: self.ctx.clone(),
						msg: self.msg.clone(),
					},	 
				);
			}
			None => {
				leave(self.ctx, self.msg).await;
			}
		}
	}
}


// si l'audio Manager jouer un son 						=> etat = playing
// si l'audio manager attend un nouveau call de poomp 	=> etat = pending/waiting
// si l'audio manager ne fait rien 						=> etat = idle