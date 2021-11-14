use serenity::framework::standard::{Args, HelpOptions,CommandGroup, CommandResult, help_commands, macros::help};
use serenity::prelude::*;
use serenity::model::prelude::*;

use std::collections::HashSet;


#[help]
#[individual_command_tip = "If you want more informations about a specific command, just pass the command as argument"]
#[command_not_found_text = "Could not find {}."]
#[max_levenshtein_distance(3)]
// #[indention_prefix = "|"]
#[embed_success_colour("#33ddff")]
#[lacking_permissions = "Hide"]
#[lacking_role = "Nothing"]
#[wrong_channel = "Strike"]
async fn my_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}