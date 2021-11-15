use serenity::framework::standard::macros::hook;
use serenity::model::prelude::*;
use serenity::prelude::*;


#[hook]
pub async fn unknown_command(_ctx: &Context, _msg: &Message, unknown_command_name: &str) {
    println!("Could not find command named '{}'", unknown_command_name);
}