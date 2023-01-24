pub mod prelude;
pub mod messages;

pub use macros::handler;

pub type HandlerType<T> = teloxide::prelude::Handler<
    'static,
    teloxide::prelude::DependencyMap,
    T,
    teloxide::dispatching::DpHandlerDescription,
>;

pub trait Handler<T>
where
    T: Sync + Send,
    Self::Output: Send + Sync,
    Self::Output: 'static,
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
