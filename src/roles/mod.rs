use teloxide::types::{CallbackQuery, Message, UserId};

use crate::handlers::HandlerType;

pub mod arash;
pub mod player;

#[derive(Debug, Clone)]
pub enum RawRole {
    Arash,
    Player,
}

#[derive(Debug, Clone)]
pub struct IdentifyCredit(UserId);

impl IdentifyCredit {
    pub fn user_id(&self) -> UserId {
        self.0
    }
}

impl From<IdentifyCredit> for u64 {
    fn from(value: IdentifyCredit) -> Self {
        value.0 .0
    }
}

impl PartialEq<u64> for IdentifyCredit {
    fn eq(&self, other: &u64) -> bool {
        self.0 .0 == *other
    }
}

pub trait Role: Clone + Send + Sync + Sized + 'static {
    const TAG: &'static str;
    const RAW: RawRole;

    fn map_identify<T>() -> HandlerType<T>
    where
        T: 'static;

    fn identify_message<T>() -> HandlerType<T>
    where
        T: 'static,
    {
        teloxide::dptree::filter_map(|msg: Message| {
            let user = msg.from()?;
            Some(IdentifyCredit(user.id))
        })
        .chain(Self::map_identify())
    }

    fn identify_callback_query<T>() -> HandlerType<T>
    where
        T: 'static,
    {
        teloxide::dptree::filter_map(|callback: CallbackQuery| {
            let user = callback.from;
            Some(IdentifyCredit(user.id))
        })
        .chain(Self::map_identify())
    }
}
