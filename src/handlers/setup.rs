use teloxide::{
    dispatching::{MessageFilterExt, UpdateFilterExt},
    types::{Message, Update},
};

use crate::extensions::handlers::HandlerExt;

use super::{
    messages::{register, role, search, start, unexpected::UnexpectedMessageHandler},
    HandlerType,
};

pub use super::messages::register::RegisterState;

pub fn setup() -> HandlerType<anyhow::Result<()>> {
    teloxide::dptree::entry().branch(
        Update::filter_message()
            .chain(Message::filter_text())
            .branch(role::branch())
            .branch(search::branch())
            .branch(start::branch())
            .branch(register::branch())
            .message_handler_branch::<UnexpectedMessageHandler>(),
    )
}
