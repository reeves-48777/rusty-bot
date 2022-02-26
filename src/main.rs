mod bot;

use bot::Bot;
use bot::core::CommonPlugin;
use bot::core::InfoPlugin;

fn main() {
    let _bot = Bot::new("DISCORD_BOT_TOKEN")
        .register(CommonPlugin)
        .register(InfoPlugin)
        .run();
}