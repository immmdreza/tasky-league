pub mod database;
pub mod extensions;
pub mod handlers;
pub mod roles;

use database::context::DbContext;

use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::*;

use crate::handlers::setup;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::PgPool::connect(&url).await?;
    let ctx = DbContext::new(pool);

    let bot = Bot::from_env();

    Dispatcher::builder(bot, setup::setup())
        .dependencies(dptree::deps![
            ctx,
            InMemStorage::<setup::RegisterState>::new()
        ])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;

    Ok(())
}
