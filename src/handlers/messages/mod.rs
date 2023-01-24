use teloxide::{
    payloads::{SendMessage, SendMessageSetters},
    requests::{JsonRequest, Requester},
    types::{ChatId, Message},
};

use self::errors::MessageHasNoSender;

use super::Handler;

pub mod commands;
pub mod register_dialogue;
pub mod search;
pub mod unexpected;

pub trait MessageHandler: Handler<Message> {
    /// Chat which this message is sent to.
    fn chat_id(&self) -> ChatId {
        self.update().chat.id
    }

    /// Message id of this message.
    fn message_id(&self) -> teloxide::types::MessageId {
        self.update().id
    }

    fn sender(&self) -> Result<&teloxide::types::User, MessageHasNoSender> {
        self.update().from().ok_or(MessageHasNoSender)
    }

    fn sender_id(&self) -> Result<u64, MessageHasNoSender> {
        Ok(self.sender()?.id.0)
    }

    /// Send a message to the chat which this message is sent.
    fn send_text(&self, text: impl Into<String>) -> JsonRequest<SendMessage> {
        self.bot().send_message(self.chat_id(), text)
    }

    /// Send a message to the chat which this message is sent as a reply to it.
    fn reply_text(&self, text: impl Into<String>) -> JsonRequest<SendMessage> {
        self.send_text(text)
            .reply_to_message_id(self.message_id())
            .allow_sending_without_reply(true)
    }
}

impl<T> MessageHandler for T where T: Handler<Message> {}

pub mod errors {
    use thiserror::Error;

    #[derive(Debug, Error)]
    #[error("The message has no sender to fetch.")]
    pub struct MessageHasNoSender;
}
