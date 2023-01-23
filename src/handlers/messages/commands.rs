use macros::handler;
use teloxide::macros::BotCommands;

use super::MessageHandler;

#[derive(Debug, BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Start,
}

#[handler(for = "Message", param = "Command")]
pub async fn commands(ctx: CommandsMessageHandler) -> anyhow::Result<()> {
    match ctx.command {
        Command::Start => {
            ctx.reply_text("Started now!").await?;
        }
    }

    Ok(())
}
