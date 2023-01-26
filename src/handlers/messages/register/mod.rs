//! Handler for registering as player and juror.

use crate::{extensions::handlers::HandlerExt, handlers::prelude::*};

mod juror;
mod not_a_player;
mod player;

pub use not_a_player::RegisterState;

pub fn branch() -> DefaultHandlerType {
    dptree::entry()
        .branch(
            dptree::filter(|text: String| text == "Register")
                .message_handler_branch::<player::PlayerMessageHandler>()
                .branch(not_a_player::branch()),
        )
        .branch(dptree::filter(|text: String| text == "Sign for Juror").chain(juror::branch()))
}
