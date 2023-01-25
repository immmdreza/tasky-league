use teloxide::{
    dispatching::DpHandlerDescription,
    prelude::{DependencyMap, Handler},
    types::Message,
};

pub trait HandlerExt<Output>
where
    Output: 'static + Send + Sync,
{
    fn handler_branch<H, T>(self) -> Self
    where
        H: crate::handlers::Handler<T, Output = Output>,
        T: 'static + Sync + Send;

    fn message_handler_branch<H>(self) -> Self
    where
        Self: Sized,
        H: crate::handlers::Handler<Message, Output = Output>,
    {
        self.handler_branch::<H, Message>()
    }
}

impl<Output> HandlerExt<Output> for Handler<'static, DependencyMap, Output, DpHandlerDescription>
where
    Output: 'static + Send + Sync,
{
    fn handler_branch<H, T>(self) -> Self
    where
        T: 'static + Sync + Send,
        H: crate::handlers::Handler<T, Output = Output>,
    {
        self.branch(H::branch())
    }
}
