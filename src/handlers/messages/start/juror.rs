use crate::{
    handlers::prelude::*,
    roles::{juror::JurorRole, Role},
};

#[handler(for = "Message")]
pub async fn juror(ctx: JurorMessageHandler) -> anyhow::Result<()> {
    ctx.reply_text("What's up partner?").await?;
    Ok(())
}

fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    JurorRole::identify_message()
}
