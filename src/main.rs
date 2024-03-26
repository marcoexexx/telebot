use teloxide::prelude::*;
use teloxide::utils::command::{self, BotCommands};

#[derive(Clone, command::BotCommands)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username")]
    Username(String),
    #[command(description = "handle a username and age", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}"))
                .await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}"),
            )
            .await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}
