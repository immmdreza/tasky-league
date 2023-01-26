pub(self) mod messages;
pub(self) mod prelude;
pub mod setup;

use teloxide::{dispatching::DpHandlerDescription, prelude::DependencyMap};

pub use macros::handler;
pub use teloxide::dptree;

pub type HandlerType<Output> =
    teloxide::prelude::Handler<'static, DependencyMap, Output, DpHandlerDescription>;

pub type DefaultHandlerReturnType = anyhow::Result<()>;

pub type DefaultHandlerType = HandlerType<DefaultHandlerReturnType>;

pub trait Handler<T>
where
    T: 'static + Sync + Send,
    Self::Output: 'static + Send + Sync,
{
    type Output;

    fn bot(&self) -> &teloxide::Bot;

    fn update(&self) -> &T;

    fn get_filter() -> HandlerType<Self::Output>;

    fn branch() -> HandlerType<Self::Output>;
}

pub fn pass_filter<T>() -> HandlerType<T>
where
    T: 'static + Sync + Send,
{
    teloxide::dptree::filter(|| true)
}
