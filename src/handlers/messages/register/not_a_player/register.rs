use crate::{extensions::SendMessageSettersExt, handlers::prelude::*};

use super::{RegisterDialogue, RegisterState};

#[handler(for = "Message", param = "RegisterDialogue")]
pub(super) async fn register(ctx: RegisterMessageHandler) -> anyhow::Result<()> {
    ctx.reply_text("Ok let me register you son ... or daughter maybe?")
        .await?;
    ctx.reply_text("What's your gender?")
        .keyboard_buttons(vec![vec!["Male", "Female"], vec!["None binary"]])
        .await?;
    ctx.register_dialogue
        .update(RegisterState::RequestedGender)
        .await?;

    Ok(())
}

fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    teloxide::dptree::case![RegisterState::Start]
}
