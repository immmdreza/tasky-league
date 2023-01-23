use macros::handler;

use crate::handlers::HandlerType;

#[handler(for = "Message")]
pub async fn register(_ctx: RegisterMessageHandler) -> anyhow::Result<()> {
    println!("Register requested!");
    Ok(())
}

fn filter_builder() -> HandlerType<anyhow::Result<()>> {
    teloxide::dptree::filter(|text: String| text == "Register")
}
