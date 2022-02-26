pub mod core;

mod traits;

use chrono::Utc;
use traits::Plugin;
use std::env;

type UUID = i64;

#[allow(dead_code)]
/// struct that contains the bot logic
pub struct Bot {
	id: UUID,
	token: Box<String>,
	plugins: Vec<Box<dyn Plugin>>
}

impl Bot {
	/// Inits the bot with a new ID and an empty Vector of Plugin
	pub fn new(token_env_var: &str) -> Self {
		Self {
			id: Utc::now().timestamp_nanos(),
			token: Box::new(env::var(token_env_var).unwrap()),
			plugins: Vec::new(),
		}
	}

	/// the bot now have all the informations it needs and will run
	pub fn run(&self) {
		for item in self.plugins.iter() {
			item.run();
		}
	}

	/// add the passed plugin to the plugins vec
	pub fn register(&mut self, plugin: impl Plugin + 'static ) -> &mut Self {
		self.plugins.push(Box::new(plugin));
		self
	}
}