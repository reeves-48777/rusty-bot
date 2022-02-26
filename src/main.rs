mod bot;

use bot::Bot;

fn main() {
    let bot = Bot::create_bot("DISCORD_BOT_TOKEN").run();
}