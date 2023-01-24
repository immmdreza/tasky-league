use crate::database::{DbContext, Model, PlayerRepo};

use super::{IdentifyCredit, Role};

#[derive(Debug, Clone)]
pub struct PlayerRole {
    player_id: i64,
}

impl PlayerRole {
    pub fn player_id(&self) -> i64 {
        self.player_id
    }
}

impl Role for PlayerRole {
    const TAG: &'static str = "player";

    fn map_identify<T>() -> crate::handlers::HandlerType<T>
    where
        T: 'static,
    {
        teloxide::dptree::filter_map_async(|ident: IdentifyCredit, db: DbContext| async move {
            let telegram_id = ident.user_id().0 as i64;
            let players: PlayerRepo = db.get();

            let player = players.get_by_telegram_id(telegram_id).await.ok()?;
            player.map(|player| PlayerRole {
                player_id: *player.get_id(),
            })
        })
    }
}
