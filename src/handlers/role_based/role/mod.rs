mod arash;
mod no_role;

use teloxide::{filter_command, macros::BotCommands};

use crate::handlers::{Handler, HandlerType};

use self::{arash::ArashRoleMessageHandler, no_role::NoRoleMessageHandler};

#[derive(Debug, Clone, BotCommands)]
pub enum RoleCommand {
    Role,
}

pub fn branch() -> HandlerType<anyhow::Result<()>> {
    filter_command::<RoleCommand, _>()
        .branch(ArashRoleMessageHandler::branch())
        .branch(NoRoleMessageHandler::branch())
}
