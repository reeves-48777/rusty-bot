mod traits;

use chrono::Utc;
use traits::Plugin;
use std::env;
use std::rc::{Rc, Weak};

type UUID = i64;

/// struct that contains the bot logic
#[derive(Clone)]
pub struct Bot {
	id: UUID,
	token: String,
	plugins: Rc<Vec<Box<dyn Plugin>>>
}

impl Bot {
	/// Inits the bot
	/// with a new ID
	/// and an empty Vector of Plugin
	fn new() -> Self {
		Self {
			id: Utc::now().timestamp_nanos(),
			token: String::from("undefined"),
			plugins: Rc::new(Vec::<Box<dyn Plugin>>::new()),
		}
	}
	fn with_token(&mut self, token: &str) -> Self {
		self.token = env::var(token).unwrap();
		*self
	}
	fn register(&mut self, plugin: Box<dyn Plugin>) -> Self {
		self.plugins.push(plugin);
		*self
	}

	pub fn create_bot(token_env_var :&str) -> Self {
		Self::new().with_token(token_env_var)
	}

	/// the bot now have all the informations it needs and will run
	pub fn run(&self) {
		todo!()
	}
}