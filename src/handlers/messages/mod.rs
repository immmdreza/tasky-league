use teloxide::{
    payloads::{SendMessage, SendMessageSetters},
    requests::{JsonRequest, Requester},
    types::{ChatId, Message},
};

use super::Handler;

pub mod commands;
pub mod unexpected;

pub trait MessageHandler: Handler<Message> {
    fn chat_id(&self) -> ChatId {
        self.update().chat.id
    }

    fn message_id(&self) -> teloxide::types::MessageId {
        self.update().id
    }

    fn send_text(&self, text: impl Into<String>) -> JsonRequest<SendMessage> {
        self.bot().send_message(self.chat_id(), text)
    }

    fn reply_text(&self, text: impl Into<String>) -> JsonRequest<SendMessage> {
        self.send_text(text)
            .reply_to_message_id(self.message_id())
            .allow_sending_without_reply(true)
    }
}

impl<T> MessageHandler for T where T: Handler<Message> {}
