use crate::{extensions::SendMessageSettersExt, handlers::prelude::*};

#[handler(for = "Message")]
pub async fn not_player(ctx: NotPlayerMessageHandler) -> anyhow::Result<()> {
    ctx.reply_text("You must first register as a player!")
        .single_keyboard_button("Register")
        .await?;

    Ok(())
}

fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    pass_filter()
}
