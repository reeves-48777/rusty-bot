use super::super::traits::Plugin;
use super::super::core::infos::BotInfo;
use super::super::Bot;

pub struct CommonPlugin;
impl Plugin for CommonPlugin {
	fn run(&self) {
		println!("Hello world");
	}
}

pub struct InfoPlugin {
	bot_infos: BotInfo,
}

impl InfoPlugin {
	fn new(bot: Bot) -> Self {
		Self {
			bot_infos: BotInfo::new(bot)
		}
	}
}
impl Plugin for InfoPlugin {
	fn run(&self) {
		println!("Informations::::::")
	}
}
