pub mod poomp;


#[group]
#[commands(poomp)]
#[description = "Audio commands for the bot"]
#[only_in(guilds)]
struct Audio;