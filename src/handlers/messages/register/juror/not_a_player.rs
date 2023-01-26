use crate::{extensions::SendMessageSettersExt, handlers::prelude::*};

#[handler(for = "Message")]
pub async fn not_player(ctx: NotPlayerMessageHandler) -> DefaultHandlerReturnType {
    ctx.reply_text("You'r not even a player yet! Please Register first.")
        .single_keyboard_button("Register")
        .await?;
    Ok(())
}

fn filter_builder() -> DefaultHandlerType {
    pass_filter()
}
