use teloxide::{
    payloads::SendMessage,
    requests::{JsonRequest, Requester},
    types::{ChatId, Message},
};

use super::Handler;

pub mod commands;

pub trait MessageHandler: Handler<Message> {
    fn chat_id(&self) -> ChatId {
        self.update().chat.id
    }

    fn reply_text(&self, text: impl Into<String>) -> JsonRequest<SendMessage> {
        self.bot().send_message(self.chat_id(), text)
    }
}

impl<T> MessageHandler for T where T: Handler<Message> {}
