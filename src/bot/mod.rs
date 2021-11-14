pub mod audio;
pub mod config;
pub mod storage;
pub mod handler;

pub use audio::AudioManager;
pub use config::{ ConfigBuilder, ConfigStore, Configuration };
pub use handler::Handler;

pub struct Bot<'a> {
	audio_manager: AudioManager<'a>,
	handler: Handler,
	config: Configuration
}

// TODO add a StorageManager struct that will fetch assets from the google drive api
// NOTE this is the best way since the repos will be lighter and contains less bloat
// each file will be accessible by all bots that will call this and the management of those will be better
// NOTE Imo it is better to do a lookup on this directory so that each time it will be modified the bot will perform
// a hot reload of the assets (like removing/adding the concerned  file from/to the cache)