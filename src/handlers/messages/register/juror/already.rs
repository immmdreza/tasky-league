use crate::{
    handlers::prelude::*,
    roles::{juror::JurorRole, Role},
};

#[handler(for = "Message")]
pub async fn already(ctx: AlreadyMessageHandler) -> DefaultHandlerReturnType {
    ctx.reply_text("You'r already a juror boss!").await?;
    Ok(())
}

fn filter_builder() -> DefaultHandlerType {
    pass_filter()
}
