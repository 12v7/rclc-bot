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

            let text = match question {
                "/start" => "Calculator bot. There are a few examples of its use:\n(1+2)(2+9)\n1\\2+3\\4\nsqrt(-4)\nfib(1000)\n0b100000000 + 0xFF".to_owned(),
                _ => match parse::eval(question, &mut cstate) {
                    Ok(val) => format!("{}", val),
                    Err(err) => format!("❌ Error\n{}", err),
                }
            };

            bot.send_message(msg.chat.id, text).await?;
        }
        Ok(())
    })
    .await;
}
