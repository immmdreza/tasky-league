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
    pub async fn get_by_telegram_id(&self, telegram_id: i64) -> anyhow::Result<Player> {
        let player = sqlx::query_as!(
            Player,
            "SELECT * FROM players WHERE telegram_id = $1",
            telegram_id
        )
        .fetch_one(self.get_pool())
        .await?;

        Ok(player)
    }
}

#[cfg(test)]
mod tests {
    use crate::database::repo::Repo;

    use super::*;

    #[tokio::test]
    async fn test_name() {
        dotenvy::dotenv().unwrap();

        let url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::PgPool::connect(&url).await.unwrap();
        let player_repo: PlayerRepo = (&pool).into();

        let inserted = player_repo
            .insert(PlayerInsertion { telegram_id: 123 })
            .await
            .unwrap();

        // Should fail if twice
        player_repo
            .update_by_id(
                inserted,
                PlayerUpdating {
                    telegram_id: crate::database::repo::Value::Set(789),
                },
            )
            .await
            .unwrap()
    }
}
