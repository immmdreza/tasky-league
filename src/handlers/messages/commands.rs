use teloxide::{filter_command, macros::BotCommands};

use crate::handlers::*;

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

pub fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    filter_command::<Command, _>()
}
