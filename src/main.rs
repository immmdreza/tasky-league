pub mod database;
pub mod extensions;
pub mod handlers;

use database::context::DbContext;
use handlers::{
    messages::{
        commands::CommandsMessageHandler, register_dialogue, unexpected::UnexpectedMessageHandler,
    },
    Handler,
};
use teloxide::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::PgPool::connect(&url).await?;
    let ctx = DbContext::new(pool);

    let bot = Bot::from_env();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .branch(CommandsMessageHandler::branch())
            .branch(
                Message::filter_text()
                    // Resolved
                    .branch(UnexpectedMessageHandler::branch())
                    .branch(register_dialogue::branch()),
            ),
    )
    .dependencies(dptree::deps![ctx])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;

    Ok(())
}
