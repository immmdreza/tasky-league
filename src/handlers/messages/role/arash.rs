use crate::{
    handlers::prelude::*,
    roles::{arash::Arash, Role},
};

#[handler(for = "Message")]
pub async fn arash_role(ctx: ArashRoleMessageHandler) -> anyhow::Result<()> {
    ctx.reply_text("Hello Arash!").await?;

    Ok(())
}

fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    Arash::identify_message()
}
