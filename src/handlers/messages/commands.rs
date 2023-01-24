use teloxide::filter_command;
use teloxide::macros::BotCommands;

use crate::extensions::SendMessageSettersExt;
use crate::handlers::prelude::*;

#[derive(Debug, BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Start,
}

#[handler(for = "Message", param = "Command", param = "DbContext")]
pub async fn commands(ctx: CommandsMessageHandler) -> anyhow::Result<()> {
    let players_repo: PlayerRepo = ctx.db_context.get();
    let jurors_repo: JurorRepo = ctx.db_context.get();
    match ctx.command {
        Command::Start => {
            let player = players_repo
                .get_by_telegram_id(ctx.sender_id()? as i64)
                .await?;
            if let Some(player) = player {
                if jurors_repo.is_juror(*player.get_id()).await? {
                    ctx.reply_text("What's up partner?").await?;
                } else {
                    ctx.reply_text("Welcome back Player!")
                        .single_keyboard_button("Sign for Juror")
                        .await?;
                }
            } else {
                ctx.reply_text("Welcome stranger! Would you mind registering your self?")
                    .single_keyboard_button("Register")
                    .await?;
            };
        }
    }

    Ok(())
}

pub fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    filter_command::<Command, _>()
}
