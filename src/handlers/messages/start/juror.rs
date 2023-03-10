use crate::{
    handlers::prelude::*,
    roles::{juror::JurorRole, Role},
};

#[handler(for = "Message")]
pub async fn juror(ctx: JurorMessageHandler) -> DefaultHandlerReturnType {
    ctx.reply_text("What's up partner?").await?;
    Ok(())
}

fn filter_builder() -> DefaultHandlerType {
    JurorRole::identify_message()
}
