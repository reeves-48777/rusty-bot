use serenity::prelude::Context;
use serenity::model::prelude::Message;
use serenity::framework::standard::{macros::command, CommandResult, Args};
use serenity::utils::MessageBuilder;



use std::num::IntErrorKind;
#[command]
#[aliases("f")]
#[description = "bot will ping the mentionned user x time"]
#[usage = "@User(s) x"]
#[example = "@User x"]
#[example = "@User1 @User2 x"]
#[min_args(2)]
// NOTE: may check if caching can improve flood (like not 'blocking' after 5 messages) 
async fn flood(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let floods = args.raw().last().unwrap().parse::<i32>();

    match floods {
        Ok(n) => {
            let targets = &msg.mentions;
            
            for _ in 0..n {
                for target in targets {
                    let content = MessageBuilder::new().push(target).build();
                    msg.channel_id.say(&ctx.http, content).await?;
                }
            }
        },

        Err(e) => {
            match e.kind() {
                IntErrorKind::Empty | IntErrorKind::InvalidDigit => {
                    msg.channel_id.say(&ctx.http, "Il faut saisir un nombre apres les mentions pour flood mec").await?;
                },
                _ => {
                    eprintln!("An error occured: {}", e);
                }
            }
        }
    }
    Ok(())
}
