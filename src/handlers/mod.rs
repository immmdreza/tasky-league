pub mod messages;

pub use macros::handler;
pub use teloxide::Bot;

// pub type ReturnType = Result<(), Box<dyn Error + Sync + Send>>;

pub trait Handler<T>
where
    T: Sync + Send,
{
    fn bot(&self) -> &Bot;

    fn update(&self) -> &T;
}
