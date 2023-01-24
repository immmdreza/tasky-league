use crate::{extensions::SendMessageSettersExt, handlers::prelude::*};

use super::{RegisterDialogue, RegisterState};

#[handler(for = "Message", param = "DbContext", param = "RegisterDialogue")]
pub(super) async fn register(ctx: RegisterMessageHandler) -> anyhow::Result<()> {
    let players: PlayerRepo = ctx.db_context.get();
    let players_info: PlayerInfoRepo = ctx.db_context.get();

    let telegram_id = ctx.sender_id()? as i64;
    let player = players.get_by_telegram_id(telegram_id).await?;
    if let Some(player) = player {
        let gender = players_info
            .get_gender_by_player_id(*player.get_id())
            .await?;

        let mut text = "You're already registered".to_string();
        text.push_str(match gender {
            players_info_repo::Gender::Male => " son!",
            players_info_repo::Gender::Female => " daughter!",
            players_info_repo::Gender::NoneBinary => " :)",
        });

        ctx.reply_text(text).await?;
    } else {
        ctx.reply_text("Ok let me register you son or daughter maybe?")
            .await?;
        ctx.reply_text("What's your gender?")
            .keyboard_buttons(vec![vec!["Male", "Female"], vec!["None binary"]])
            .await?;
        ctx.register_dialogue
            .update(RegisterState::RequestedGender)
            .await?
    }

    Ok(())
}

fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    teloxide::dptree::case![RegisterState::Start]
}
