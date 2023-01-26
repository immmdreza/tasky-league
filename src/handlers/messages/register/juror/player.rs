use crate::{handlers::prelude::*, roles::player::PlayerRole};

#[handler(for = "Message", param = "PlayerRole", param = "DbContext")]
pub async fn player(ctx: PlayerMessageHandler) -> DefaultHandlerReturnType {
    let jurors: JurorRepo = ctx.db_context.get();

    jurors
        .insert(JurorInsertion {
            telegram_id: ctx.player_role.telegram_id(),
            player_id: ctx.player_role.player_id(),
            ..Default::default()
        })
        .await?;
    ctx.reply_text("Congrats! You're a juror now.").await?;

    Ok(())
}

fn filter_builder() -> DefaultHandlerType {
    pass_filter()
}
