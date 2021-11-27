use super::Configuration;

/// Builder pattern for Configuration
pub struct ConfigBuilder {
	clear_command_calls: Option<bool>,
	mute_bot: Option<bool>,
	flood_delay: Option<f32>,
}

impl ConfigBuilder {
	/// Constructor for ConfigBuilder
	pub fn new() -> Self {
		Self {
			clear_command_calls: None,
			mute_bot: None,
			flood_delay: None
		}
	}
	/// Configure whether the command calls should be clear or not
	pub fn clear_calls(&mut self, new_value: bool) -> &mut Self{
		self.clear_command_calls = Some(new_value);
		self
	}
	/// Configure whether the bot has to be mute in voice channel
	pub fn mute_bot(&mut self, new_value: bool) -> &mut Self {
		self.mute_bot = Some(new_value);
		self
	}
	/// Sets the delay for the flood command
	pub fn flood_delay(&mut self, new_value: Option<f32>) -> &mut Self {
		self.flood_delay = new_value;
		self
	}

	/// Build function
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