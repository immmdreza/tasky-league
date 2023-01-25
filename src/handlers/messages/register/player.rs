use crate::{database::players_info_repo::Gender, handlers::prelude::*, roles::player::PlayerRole};

#[handler(
    for = "Message",
    param = "PlayerRole",
    param = "DbContext",
    param = "Gender"
)]
pub async fn player(ctx: PlayerMessageHandler) -> anyhow::Result<()> {
    ctx.reply_text(format!("You're already registered {}", ctx.gender.to_fun()))
        .await?;

    Ok(())
}

fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    PlayerRole::map_message_with_gender()
}
