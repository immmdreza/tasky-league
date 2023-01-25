use std::str::FromStr;

use super::{RegisterDialogue, RegisterState};
use crate::{
    database::players_info_repo::Gender,
    extensions::SendMessageSettersExt,
    handlers::{prelude::*, Handler},
};

#[handler(for = "Message", param = "RegisterDialogue", param = "DbContext")]
pub(super) async fn received_gender(ctx: ReceivedGenderMessageHandler) -> anyhow::Result<()> {
    if let Some(text) = ctx.update().text() {
        match text {
            "Male" | "Female" | "None binary" => {
                let gender = Gender::from_str(text)?;
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

                ctx.reply_text(format!("Everything is done {}", gender.to_fun()))
                    .remove_keyboards()
                    .await?;
            }
            "Cancel" => {
                ctx.register_dialogue.exit().await?;
                ctx.reply_text("Ok rest in peace!")
                    .remove_keyboards()
                    .await?;
            }
            _ => {
                ctx.reply_text("Please send me a gender or Cancel.").await?;
            }
        }
    } else {
    };

    Ok(())
}

fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    teloxide::dptree::case![RegisterState::RequestedGender]
}
