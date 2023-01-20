use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&url).await?;

    // Inserting players
    let inserted = sqlx::query!(
        r#"--sql
        DELETE FROM players
        "#
    )
    .execute(&pool)
    .await?;

    Ok(())
}
