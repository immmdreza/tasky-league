use teloxide::{
    payloads::{SendMessage, SendMessageSetters},
    requests::{JsonRequest, Requester},
    types::{ChatId, Message},
};

use super::Handler;

pub mod commands;
pub mod register;
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

    fn sender_id(&self) -> Option<u64> {
        self.update().from()?.id.0.into()
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
