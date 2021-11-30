use bot::CommonBot;
use bot::Bot;


#[tokio::main]
async fn main() {
    let bot = CommonBot::new();
    bot.run().await;
    //tracing_subscriber::fmt::init();
}

// fn _check_msg(result: SerenityResult<Message>) {
//     if let Err(why) = result {
//         println!("Error sending message: {:?}", why);
//     }
// }