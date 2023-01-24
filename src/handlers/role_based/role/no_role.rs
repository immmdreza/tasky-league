use crate::handlers::prelude::*;

#[handler(for = "Message")]
pub async fn no_role(ctx: NoRoleMessageHandler) -> anyhow::Result<()> {
    ctx.reply_text("Hello no role!").await?;

    Ok(())
}

fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    pass_filter()
}
