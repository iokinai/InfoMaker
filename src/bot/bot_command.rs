use teloxide::macros::BotCommands;

#[derive(BotCommands)]
#[command(rename_rule = "lowercase")]
pub enum BotCommand {
    #[command(description = "start the bot.")]
    Start,
    #[command(description = "display help message")]
    Help,
}
