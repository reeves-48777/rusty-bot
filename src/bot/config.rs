// TODO work on the implementation of the configuration,
// this struct will be used for the config ** command (e.g: `config set`)
// examples:
// 		$config set bot_muted true/false
// 		$config set clear_calls true/false
//		$config set flood_delay xxx

/// Contains bot settings
/// 
/// clear_calls: used to check if the command calls 
/// like ($ping) have to be cleared after the command is executed or not
/// 
/// muted: the name here is pretty relevant, says whether or not the bot is muted
/// 
/// flood_delay: (experimental, might be deprecated in the future)
/// used to set a delay for the flood command, due to discord api rate limit,
/// this delay could be useful in order to flood people without having the phenomenon of messages that freeze by batch of 5
pub struct Configuration {
	clear_calls: bool,
	muted: bool,
	flood_delay: f32
}

impl Configuration {
	pub fn new() -> Self {
		Self {
			clear_calls: false,
			muted: false,
			flood_delay: 0.0
		}
	}
	/// Returns if the commands calls are cleared
	pub fn get_clear_calls(&self) -> bool {
		self.clear_calls
	}
	/// Sets whether the commmand calls have to be cleared or not
	/// # Examples
	/// ```
	/// let mut config = Configuration::new();
	/// // default value for clear_command_calls is false;
	/// assert_eq!(false, config.get_clear_command_calls());
	/// config.set_clear_command_calls(true);
	/// assert_eq!(true, config.get_clear_command_calls());
	/// ```
	pub fn clear_calls(&mut self, new_value: bool) {
		self.clear_calls = new_value;
	}

	pub fn muted(&self) -> bool {
		self.muted
	}
	pub fn mute(&mut self, new_value: bool) {
		self.muted = new_value;
	}

	pub fn get_flood_delay(&self) -> f32 {
		self.flood_delay
	}
	pub fn set_flood_delay(&mut self, new_value: f32) {
		self.flood_delay = new_value;
	}
}

impl PartialEq for Configuration {
	fn eq(&self, other: &Self) -> bool {
		self.clear_calls == other.get_clear_calls() && self.muted == other.muted() && self.flood_delay == other.get_flood_delay()
	}
}

impl Default for Configuration {
	fn default() -> Self {
		Self::new()
	}
}

pub struct ConfigBuilder {
	clear_command_calls: Option<bool>,
	mute_bot: Option<bool>,
	flood_delay: Option<f32>,
}

impl ConfigBuilder {
	pub fn new() -> Self {
		Self {
			clear_command_calls: None,
			mute_bot: None,
			flood_delay: None
		}
	}

	pub fn clear_calls(&mut self, new_value: bool) -> &mut Self{
		self.clear_command_calls = Some(new_value);
		self
	}
	pub fn mute_bot(&mut self, new_value: bool) -> &mut Self {
		self.mute_bot = Some(new_value);
		self
	}
	pub fn flood_delay(&mut self, new_value: Option<f32>) -> &mut Self {
		self.flood_delay = new_value;
		self
	}

	pub fn build(&self) -> Configuration {
		let mut new_conf = Configuration::new();

		if let Some(clr) = self.clear_command_calls {
			new_conf.clear_calls(clr)
		}
		if let Some(mute) = self.mute_bot {
			new_conf.mute(mute);
		}
		if let Some(float) = self.flood_delay {
			new_conf.set_flood_delay(float);
		}
		new_conf
	}
}

use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;
pub struct ConfigStore;

impl TypeMapKey for ConfigStore {
    type Value = Arc<RwLock<Configuration>>;
}