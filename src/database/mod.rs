pub mod context;
pub mod repo;
pub mod repos;

pub use macros::Model;
use sqlx::PgPool;

pub trait Db<'r> {
    fn get_pool(&'r self) -> &'r PgPool;
}
