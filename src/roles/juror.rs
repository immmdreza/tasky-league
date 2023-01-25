use crate::database::{DbContext, JurorRepo};

use super::{IdentifyCredit, RawRole, Role};

#[derive(Debug, Clone)]
pub struct JurorRole {
    id: i64,
    player_id: i64,
}

impl JurorRole {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn player_id(&self) -> i64 {
        self.player_id
    }
}

impl Role for JurorRole {
    const TAG: &'static str = "juror";
    const RAW: RawRole = RawRole::Juror;

    fn map_identify<T>() -> crate::handlers::HandlerType<T>
    where
        T: 'static,
    {
        teloxide::dptree::filter_map_async(|ident: IdentifyCredit, db: DbContext| async move {
            let telegram_id = ident.user_id().0 as i64;
            let players: JurorRepo = db.get();

            let player = players
                .get_lite_juror_by_telegram_id(telegram_id)
                .await
                .ok()?;

            Some(JurorRole {
                player_id: player.player_id(),
                id: player.id(),
            })
        })
    }
}
