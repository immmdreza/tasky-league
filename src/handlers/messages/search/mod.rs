use teloxide::{filter_command, macros::BotCommands};

use crate::handlers::{Handler, HandlerType};

use self::{not_a_player::NotPlayerMessageHandler, player::SearchMessageHandler};

mod not_a_player;
mod player;

#[derive(Debug, Clone, BotCommands)]
#[command(rename_rule = "lowercase")]
enum SearchCommand {
    Search,
}

pub fn branch() -> HandlerType<anyhow::Result<()>> {
    filter_command::<SearchCommand, _>()
        .branch(SearchMessageHandler::branch())
        .branch(NotPlayerMessageHandler::branch())
}
