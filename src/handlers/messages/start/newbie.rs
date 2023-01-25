use crate::{extensions::SendMessageSettersExt, handlers::prelude::*};

#[handler(for = "Message")]
pub async fn newbie(ctx: NewbieMessageHandler) -> anyhow::Result<()> {
    ctx.reply_text("Welcome stranger! Would you mind registering your self?")
        .single_keyboard_button("Register")
        .await?;

    Ok(())
}

fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    pass_filter()
}
