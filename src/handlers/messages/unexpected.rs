use crate::handlers::*;

#[handler(for = "Message")]
pub async fn unexpected(_handler: UnexpectedMessageHandler) -> anyhow::Result<()> {
    println!("Unexpected text message received!");
    Ok(())
}

pub fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    pass_filter()
}
