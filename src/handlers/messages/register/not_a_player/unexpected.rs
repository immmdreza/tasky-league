use crate::handlers::prelude::*;

#[handler(for = "Message")]
pub(super) async fn unexpected(ctx: UnexpectedMessageHandler) -> anyhow::Result<()> {
    ctx.reply_text("Idk what are you trying to tell me, just select a gender!")
        .await?;

    Ok(())
}

fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    pass_filter()
}
