use crate::handlers::{Handler, HandlerType};

mod not_a_player;
mod player;

pub use not_a_player::RegisterState;

pub fn branch() -> HandlerType<anyhow::Result<()>> {
    teloxide::dptree::filter(|text: String| text == "Register")
        .branch(player::PlayerMessageHandler::branch())
        .branch(not_a_player::branch())
}
