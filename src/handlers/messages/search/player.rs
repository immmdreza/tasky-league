use crate::{
    handlers::prelude::*,
    roles::{player::PlayerRole, Role},
};

#[handler(for = "Message", param = "PlayerRole")]
pub async fn search(ctx: SearchMessageHandler) -> anyhow::Result<()> {
    ctx.reply_text(format!(
        "OK! searching for a match by ({})!",
        ctx.player_role.player_id()
    ))
    .await?;
    Ok(())
}

fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    PlayerRole::identify_message()
}
