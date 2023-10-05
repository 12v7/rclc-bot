use rcalc_lib::parse;
use teloxide::prelude::*;
use tokio;

const TOKEN: &str = "=== Copy your bot's token here ===";

#[tokio::main]
async fn main() {
    let bot = Bot::new(TOKEN);

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(question) = msg.text() {

            let mut cstate = parse::CalcState::new();

            let text = match parse::eval(question, &mut cstate) {
                Ok(val) => format!("{}", val),
                Err(err) => format!("❌ {} ❌", err),
            };

            bot.send_message(msg.chat.id, text).await?;
        }
        Ok(())
    })
    .await;
}
