use teloxide::{payloads::SendMessageSetters, types::KeyboardRemove};

use crate::{database::players_info_repo::Gender, handlers::prelude::*};

use super::{RegisterDialogue, RegisterState};

#[handler(
    for = "Message",
    param = "RegisterDialogue",
    param = "Gender",
    param = "DbContext"
)]
pub(super) async fn finish_register(ctx: FinishRegisterMessageHandler) -> anyhow::Result<()> {
    let players: PlayerRepo = ctx.db_context.get();
    let players_info: PlayerInfoRepo = ctx.db_context.get();

    let telegram_id = ctx.sender_id()? as i64;
    let player_id = players.insert(PlayerInsertion { telegram_id }).await?;
    players_info
        .insert(PlayerInfoInsertion {
            player_id,
            telegram_id,
            ..Default::default()
        })
        .await?;

    ctx.register_dialogue.exit().await?;

    let mut text = "Everything is done".to_string();
    text.push_str(match ctx.gender {
        players_info_repo::Gender::Male => " son!",
        players_info_repo::Gender::Female => " daughter!",
        players_info_repo::Gender::NoneBinary => " :)",
    });

    ctx.reply_text(text)
        .reply_markup(KeyboardRemove::new())
        .await?;

    Ok(())
}

fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    teloxide::dptree::case![RegisterState::ReceiveGender { gender }]
}
