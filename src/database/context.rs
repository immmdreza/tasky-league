use std::fmt::Display;

use sqlx::PgPool;

use super::repo::{Model, Repo};

#[derive(Debug, Clone)]
pub struct DbContext
// <'r, R, T, ID>
// where
//     R: Repo<'r, T, ID>,
//     T: Model<ID> + for<'m> sqlx::FromRow<'m, sqlx::postgres::PgRow> + Send + Unpin,
//     ID: Display + Send + Unpin + 'static,
{
    // _fake_1: &'r PhantomData<T>,
    // _fake_2: &'r PhantomData<ID>,
    // _fake_3: &'r PhantomData<R>,
    pool: PgPool,
}

impl DbContext
// <'r, R, T, ID>
// where
//     T: Model<ID> + for<'m> sqlx::FromRow<'m, sqlx::postgres::PgRow> + Send + Unpin,
//     ID: Display + Send + Unpin + 'static,
//     R: Repo<'r, T, ID>,
{
    pub fn new(pool: PgPool) -> Self {
        Self {
            // _fake_1: &PhantomData,
            // _fake_2: &PhantomData,
            // _fake_3: &PhantomData,
            pool,
        }
    }

    pub fn get<'r, R, T, ID>(&'r self) -> R
    where
        T: Model<ID> + for<'m> sqlx::FromRow<'m, sqlx::postgres::PgRow> + Send + Unpin,
        ID: Display + Send + Unpin + 'static,
        R: Repo<'r, T, ID>,
    {
        R::new(&self.pool)
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::database::repos::players_repo::PlayerRepo;

    #[tokio::test]
    async fn test_name() {
        dotenvy::dotenv().unwrap();

        let url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::PgPool::connect(&url).await.unwrap();

        let ctx = DbContext::new(pool);
        let _repo: PlayerRepo = ctx.get();
    }
}
