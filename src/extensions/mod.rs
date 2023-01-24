use teloxide::{
    payloads::SendMessageSetters,
    types::{KeyboardButton, KeyboardMarkup, KeyboardRemove},
};

pub trait SendMessageSettersExt: SendMessageSetters {
    fn single_keyboard_button(self, text: &str) -> Self {
        self.reply_markup(
            KeyboardMarkup::new(vec![vec![KeyboardButton::new(text)]]).resize_keyboard(true),
        )
    }

    fn keyboard_buttons<'r, T, R>(self, buttons: T) -> Self
    where
        T: IntoIterator<Item = R>,
        R: IntoIterator<Item = &'r str>,
    {
        let b = buttons
            .into_iter()
            .map(|f: R| f.into_iter().map(KeyboardButton::new));
        self.reply_markup(KeyboardMarkup::new(b).resize_keyboard(true))
    }

    fn remove_keyboards(self) -> Self {
        self.reply_markup(KeyboardRemove::new())
    }
}

impl<T> SendMessageSettersExt for T where T: SendMessageSetters {}
