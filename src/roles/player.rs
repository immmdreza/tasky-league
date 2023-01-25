use crate::database::{DbContext, Model, PlayerInfoRepo, PlayerRepo};

use super::{IdentifyCredit, RawRole, Role};

#[derive(Debug, Clone)]
pub struct PlayerRole {
    player_id: i64,
}

impl PlayerRole {
    pub fn player_id(&self) -> i64 {
        self.player_id
    }

    pub fn map_message_with_gender<T>() -> crate::handlers::HandlerType<T>
    where
        T: 'static,
    {
        Self::identify_message().chain(teloxide::dptree::filter_map_async(
            |player_role: PlayerRole, db: DbContext| async move {
                let players_info: PlayerInfoRepo = db.get();
                let gender = players_info
                    .get_gender_by_player_id(player_role.player_id())
                    .await
                    .ok()?;
                Some(gender)
            },
        ))
    }
}

impl Role for PlayerRole {
    const TAG: &'static str = "player";
    const RAW: RawRole = RawRole::Player;

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
