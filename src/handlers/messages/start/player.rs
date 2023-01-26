use crate::{
    extensions::SendMessageSettersExt,
    handlers::prelude::*,
    roles::{player::PlayerRole, Role},
};

#[handler(for = "Message")]
pub async fn player(ctx: PlayerMessageHandler) -> DefaultHandlerReturnType {
    ctx.reply_text("Welcome back Player!")
        .single_keyboard_button("Sign for Juror")
        .await?;

    Ok(())
}

fn filter_builder() -> DefaultHandlerType {
    PlayerRole::identify_message()
}
