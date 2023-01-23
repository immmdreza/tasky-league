use std::{fmt::Display, marker::PhantomData};

use super::Db;

#[derive(Debug, Default)]
pub enum Value<T> {
    Set(T),

    #[default]
    Unset,
}

pub trait Updating: Default {
    fn get_raw_update_query(&self) -> String;
}

#[allow(dead_code)]
pub struct UpdatingWrapper<'m, ID, T, M>
where
    M: Model<ID>,
    T: Updating,
    ID: Display + Send + Unpin + 'static,
{
    fake: PhantomData<ID>,
    model: &'m M,
    updating: T,
}

pub trait Model<ID>
where
    ID: Display + Send + Unpin + 'static,
{
    fn get_id(&self) -> &ID;

    fn get_wrapper<T, M>(&self) -> UpdatingWrapper<'_, ID, T, Self>
    where
        T: Updating,
        Self: Sized,
    {
        UpdatingWrapper {
            fake: PhantomData,
            model: self,
            updating: Default::default(),
        }
    }
}

#[async_trait::async_trait]
pub trait Repo<'r, T, ID>: Db<'r>
where
    T: Model<ID> + for<'m> sqlx::FromRow<'m, sqlx::postgres::PgRow> + Send + Unpin,
    ID: Display + Send + Unpin + 'static,
{
    const TABLE: &'static str;

    fn new(pool: &'r sqlx::PgPool) -> Self;

    async fn get_by_id(&'r self, id: ID) -> Result<T, sqlx::Error> {
        sqlx::query_as(&format!("SELECT * FROM {} WHERE id = {};", Self::TABLE, id))
            .fetch_one(self.get_pool())
            .await
    }

    async fn delete_by_id(&'r self, id: ID) -> Result<(), sqlx::Error> {
        sqlx::query(&format!("DELETE FROM {} WHERE id = {};", Self::TABLE, id))
            .execute(self.get_pool())
            .await?;

        Ok(())
    }

    async fn update_by_id(
        &'r self,
        id: ID,
        updating: (impl Updating + Send + Sync),
    ) -> anyhow::Result<()> {
        let query = format!("{} WHERE id = {}", updating.get_raw_update_query(), id);
        sqlx::query(&query).execute(self.get_pool()).await?;

        Ok(())
    }
}
