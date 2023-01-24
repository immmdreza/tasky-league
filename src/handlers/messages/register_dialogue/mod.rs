pub mod received_gender;
pub mod register;
pub mod unexpected;

use teloxide::{dispatching::dialogue::InMemStorage, prelude::Dialogue, types::Message};

use crate::{
    database::players_info_repo::Gender,
    handlers::{Handler, HandlerType},
};

use self::{ received_gender::ReceivedGenderMessageHandler,
    register::RegisterMessageHandler, unexpected::UnexpectedMessageHandler,
};

#[derive(Debug, Clone, Default)]
pub enum RegisterState {
    #[default]
    Start,
    RequestedGender,
    ReceiveGender {
        gender: Gender,
    },
}

pub type RegisterDialogue = Dialogue<RegisterState, InMemStorage<RegisterState>>;

pub fn branch() -> HandlerType<anyhow::Result<()>> {
    teloxide::dispatching::dialogue::enter::<Message, InMemStorage<RegisterState>, RegisterState, _>()
        .branch(RegisterMessageHandler::branch())
        .branch(ReceivedGenderMessageHandler::branch())
        .branch(UnexpectedMessageHandler::branch())
}
