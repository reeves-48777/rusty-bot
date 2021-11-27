use serenity::framework::standard::macros::group;

pub const BOT_COMMANDS: &[&str]  = &["$poomp", "$p", "$mooscles", "$m" ,"$config" ,"$conf" ,"$flood", "$f", "$help", "$clear", "$clr"];


pub mod help;
pub mod clear;
pub mod flood;
pub mod mooscles;


pub use help::MY_HELP;

#[group]
#[commands(mooscles, poomp, flood, clear)]
#[only_in(guilds)]
#[description = "General commands for the bot"]
struct General;

use clear::CLEAR_COMMAND;
use flood::FLOOD_COMMAND;
use mooscles::MOOSCLES_COMMAND;
use audio::commands::poomp::POOMP_COMMAND;


