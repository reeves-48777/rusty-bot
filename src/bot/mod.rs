pub mod audio;

pub struct Configuration {
	remove_command_msgs: bool
}

// find a way to configure this with a command for the bot
impl Configuration {
	pub fn new(remove: bool) -> Self {
		Self {
			remove_command_msgs: remove
		}
	}

	pub fn get_config(&self) -> bool {
		self.remove_command_msgs
	}
}

pub use audio::AudioManager;

// TODO add a StorageManager struct that will fetch assets from the google drive api
// NOTE this is the best way since the repos will be lighter and contains less bloat
// each file will be accessible by all bots that will call this and the management of those will be better
// NOTE Imo it is better to do a lookup on this directory so that each time it will be modified the bot will perform
// a hot reload of the assets (like removing/adding the concerned  file from/to the cache)