mod bot;

use bot::Bot;
use bot::core::CommonPlugin;

fn main() {
    // common plugin may have to be initialized when the bot is
    let _ = Bot::new("DISCORD_BOT_TOKEN")
        .register(CommonPlugin)
        .run();
}