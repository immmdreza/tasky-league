pub mod context;
pub mod repo;
pub mod repos;

pub use macros::Model;
use sqlx::PgPool;

pub use context::DbContext;
pub use repo::{Model, Repo};
pub use repos::*;

pub trait Db<'r> {
    fn get_pool(&'r self) -> &'r PgPool;
}
