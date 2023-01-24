pub mod database;
pub mod extensions;
pub mod handlers;
pub mod roles;

use database::context::DbContext;
use handlers::{
    messages::{
        commands::CommandsMessageHandler, register_dialogue, unexpected::UnexpectedMessageHandler,
    },
    role_based::role,
    Handler,
};
use teloxide::{dispatching::MessageFilterExt, prelude::*};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::PgPool::connect(&url).await?;
    let ctx = DbContext::new(pool);

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        teloxide::dptree::entry().branch(
            Update::filter_message()
                .chain(Message::filter_text())
                .branch(role::branch())
                .branch(CommandsMessageHandler::branch())
                .branch(register_dialogue::branch())
                .branch(UnexpectedMessageHandler::branch()),
        ),
    )
    .dependencies(dptree::deps![ctx])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;

    Ok(())
}
