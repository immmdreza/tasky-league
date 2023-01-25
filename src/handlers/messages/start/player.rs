use crate::{
    extensions::SendMessageSettersExt,
    handlers::prelude::*,
    roles::{player::PlayerRole, Role},
};

#[handler(for = "Message")]
pub async fn player(ctx: PlayerMessageHandler) -> anyhow::Result<()> {
    ctx.reply_text("Welcome back Player!")
        .single_keyboard_button("Sign for Juror")
        .await?;

    Ok(())
}

fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    PlayerRole::identify_message()
}
