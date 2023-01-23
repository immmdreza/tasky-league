use macros::Model;
use sqlx::FromRow;

use crate::database::Db;

#[derive(Debug, FromRow, Model)]
#[model(name = "players")]
pub struct Player {
    id: i64,
    telegram_id: i64,
}

impl Player {
    pub fn telegram_id(&self) -> i64 {
        self.telegram_id
    }
}

impl<'r> PlayerRepo<'r> {
    pub async fn is_player(&self, telegram_id: i64) -> anyhow::Result<bool> {
        let exists = sqlx::query!(
            "select exists(select 1 from players where id=$1)",
            telegram_id
        )
        .fetch_one(self.get_pool())
        .await?;

        Ok(exists.exists.unwrap_or(false))
    }

    pub async fn get_by_telegram_id(&self, telegram_id: i64) -> anyhow::Result<Option<Player>> {
        let player = sqlx::query_as("SELECT * FROM players WHERE telegram_id = $1")
            .bind(telegram_id)
            .fetch_optional(self.get_pool())
            .await?;

        Ok(player)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_name() {
        dotenvy::dotenv().unwrap();

        let url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::PgPool::connect(&url).await.unwrap();
        let player_repo: PlayerRepo = (&pool).into();

        let _player = player_repo.get_by_telegram_id(154).await.unwrap();
    }
}
