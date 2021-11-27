use serenity::framework::standard::macros::group;

pub mod set;
pub mod show;

#[group]
// Sets the prefixes config and conf for this group
// allows us to call commands that will configure / change the behaviour of the bot
// via `$config cmd` or `$conf cmd` instead of just `$`
#[prefixes("config", "conf")]
#[description = "Commands that allows you to change or see the bot's current config"]
#[commands(set,show)]
struct Config;

use set::SET_COMMAND;
use show::SHOW_COMMAND;