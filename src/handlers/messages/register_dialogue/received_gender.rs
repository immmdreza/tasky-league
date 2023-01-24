use std::str::FromStr;

use super::{RegisterDialogue, RegisterState};
use crate::{
    database::players_info_repo::Gender,
    handlers::{prelude::*, Handler},
};

#[handler(for = "Message", param = "RegisterDialogue")]
pub(super) async fn received_gender(ctx: ReceivedGenderMessageHandler) -> anyhow::Result<()> {
    if let Some(text) = ctx.update().text() {
        match text {
            "Male" | "Female" | "None binary" => {
                let gender = Gender::from_str(text)?;
                ctx.register_dialogue
                    .update(RegisterState::ReceiveGender { gender })
                    .await?;
            }
            "Cancel" => {
                ctx.register_dialogue.exit().await?;
                ctx.reply_text("Ok rest in peace!").await?;
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
