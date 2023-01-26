use crate::{
    extensions::handlers::HandlerExt,
    handlers::{prelude::*, Handler},
    roles::{juror::JurorRole, player::PlayerRole, Role},
};

mod already;
mod not_a_player;
mod player;

pub fn branch() -> DefaultHandlerType {
    dptree::entry()
        .branch(
            PlayerRole::identify_message()
                .branch(
                    JurorRole::map_message_from_player()
                        .chain(already::AlreadyMessageHandler::branch()),
                )
                .branch(player::PlayerMessageHandler::branch()),
        )
        .message_handler_branch::<not_a_player::NotPlayerMessageHandler>()
}
