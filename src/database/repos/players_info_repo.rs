use std::fmt::Display;

use crate::database::Db;
use macros::Model;
use sqlx::FromRow;
use strum_macros::EnumString;

#[derive(sqlx::Type, Debug, Default, Clone, Copy, EnumString)]
#[sqlx(type_name = "gender", rename_all = "snake_case")]
pub enum Gender {
    #[default]
    Male,
    Female,

    #[strum(serialize = "None binary")]
    NoneBinary,
}

impl Gender {
    pub fn to_fun(&self) -> String {
        (match self {
            Gender::Male => "son!",
            Gender::Female => "daughter!",
            Gender::NoneBinary => ":)",
        })
        .to_string()
    }
}

impl Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "male"),
            Gender::Female => write!(f, "female"),
            Gender::NoneBinary => write!(f, "none_binary"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct GenderKeeper {
    gender: Gender,
}

#[derive(Debug, FromRow, Model)]
#[model(name = "players_info")]
pub struct PlayerInfo {
    id: i64,
    telegram_id: i64,
    kindness: f32,
    skill: f32,
    gender: Gender,
    player_id: i64,
}

impl PlayerInfoRepo<'_> {
    pub async fn get_by_player_id(&self, player_id: i64) -> anyhow::Result<PlayerInfo> {
        let info = sqlx::query_as!(
            PlayerInfo,
            r#"select
            id, telegram_id, kindness, skill, gender as "gender: _", player_id
            from players_info where player_id = $1"#,
            player_id
        )
        .fetch_one(self.get_pool())
        .await?;

        Ok(info)
    }

    pub async fn get_gender_by_player_id(&self, player_id: i64) -> anyhow::Result<Gender> {
        let info = sqlx::query_as!(
            GenderKeeper,
            r#"select gender as "gender: _"
            from players_info where player_id = $1"#,
            player_id
        )
        .fetch_one(self.get_pool())
        .await?;

        Ok(info.gender)
    }
}
