pub mod jurors_repo;
pub mod players_info_repo;
pub mod players_repo;

pub use jurors_repo::{Juror, JurorInsertion, JurorRepo, JurorUpdating};
pub use players_info_repo::{PlayerInfo, PlayerInfoInsertion, PlayerInfoRepo, PlayerInfoUpdating};
pub use players_repo::{Player, PlayerInsertion, PlayerRepo, PlayerUpdating};
