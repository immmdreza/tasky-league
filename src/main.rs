pub mod database;
pub mod extensions;
pub mod handlers;
pub mod roles;

use database::context::DbContext;
use handlers::{
    messages::{register, start, unexpected::UnexpectedMessageHandler},
    role_based::role,
    Handler,
};

use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::{dispatching::MessageFilterExt, prelude::*};

use crate::handlers::messages::search;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    pretty_env_logger::init();
    log::info!("Starting command bot...");

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
                .branch(search::branch())
                .branch(start::branch())
                .branch(register::branch())
                .branch(UnexpectedMessageHandler::branch()),
        ),
    )
    .dependencies(dptree::deps![
        ctx,
        InMemStorage::<register::RegisterState>::new()
    ])
    .enable_ctrlc_handler()
    .build()
    .dispatch()
    .await;

    Ok(())
}
