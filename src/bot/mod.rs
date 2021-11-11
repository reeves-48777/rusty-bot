pub mod audio;

// TODO work on the implementation of the configuration,
// this struct will be used for the config ** command (e.g: `config set`)
// examples:
// 		$config set bot_muted true/false
// 		$config set clear_cmd_calls true/false
//		$config set flood_delay xxx
pub struct Configuration {
	clear_command_calls: bool,
	bot_muted: bool,
	flood_delay: Option<f32>
}

// find a way to configure this with a command for the bot
impl Configuration {
	pub fn new() -> Self {
		Self {
			clear_command_calls: false,
			bot_muted: false,
			flood_delay: None
		}
	}

	pub fn get_clear_command_calls(&self) -> bool {
		self.clear_command_calls
	}
	pub fn set_clear_command_calls(&mut self, new_value: bool) {
		self.clear_command_calls = new_value;
	}

	pub fn get_bot_muted(&self) -> bool {
		self.bot_muted
	}
	pub fn set_bot_muted(&mut self, new_value: bool) {
		self.bot_muted = new_value;
	}

	pub fn get_flood_delay(&self) -> f32 {
		self.flood_delay.unwrap()
	}
	pub fn set_flood_delay(&mut self, new_value: Option<f32>) {
		self.flood_delay = new_value;
	}
}

pub use audio::AudioManager;

// TODO add a StorageManager struct that will fetch assets from the google drive api
// NOTE this is the best way since the repos will be lighter and contains less bloat
// each file will be accessible by all bots that will call this and the management of those will be better
// NOTE Imo it is better to do a lookup on this directory so that each time it will be modified the bot will perform
// a hot reload of the assets (like removing/adding the concerned  file from/to the cache)