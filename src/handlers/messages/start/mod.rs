use teloxide::{filter_command, macros::BotCommands};

use crate::{extensions::handlers::HandlerExt, handlers::HandlerType};

mod juror;
mod newbie;
mod player;

#[derive(Debug, BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
pub enum StartCommand {
    Start,
}

pub fn branch() -> HandlerType<anyhow::Result<()>> {
    filter_command::<StartCommand, _>()
        .message_handler_branch::<juror::JurorMessageHandler>()
        .message_handler_branch::<player::PlayerMessageHandler>()
        .message_handler_branch::<newbie::NewbieMessageHandler>()
}
