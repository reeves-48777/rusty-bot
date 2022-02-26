use std::{
	rc::Rc,
	fmt::{
		self,
		Display
	}
};
use super::super::Bot;
pub struct BotInfo {
	bot: Rc<Bot>
}

impl BotInfo {
	pub fn new(bot: Bot) -> Self {
		Self {
			bot: Rc::from(bot)
		}
	}
}

impl Display for BotInfo {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "id:\t{}\ntoken:\t{}\nplugins:\t{}", self.bot.id, self.bot.token, self.bot.plugins.len())
	}
}
