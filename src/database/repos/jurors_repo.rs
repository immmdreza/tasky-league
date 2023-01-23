use macros::Model;
use sqlx::FromRow;

use crate::database::Db;

#[derive(Debug, FromRow, Model, Default)]
#[model(name = "jurors")]
pub struct Juror {
    id: i64,
    telegram_id: i64,
    kindness: f32,
    skill: f32,
    available: bool,
    player_id: i64,
}

impl Juror {
    pub fn telegram_id(&self) -> i64 {
        self.telegram_id
    }

    pub fn kindness(&self) -> f32 {
        self.kindness
    }

    pub fn skill(&self) -> f32 {
        self.skill
    }

    pub fn available(&self) -> bool {
        self.available
    }

    pub fn player_id(&self) -> i64 {
        self.player_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::database::{
        repo::{Model, Repo},
        repos::players_repo::PlayerRepo,
    };

    #[tokio::test]
    async fn test_name() {
        dotenvy::dotenv().unwrap();

        let url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::PgPool::connect(&url).await.unwrap();
        let player_repo = PlayerRepo::new(&pool);
        let juror_repo = JurorRepo::new(&pool);

        let player = player_repo.get_by_telegram_id(789).await.unwrap();

        juror_repo
            .insert(JurorInsertion {
                telegram_id: player.telegram_id(),
                player_id: player.get_id().to_owned(),
                ..Default::default()
            })
            .await
            .unwrap();
    }
}
