use super::{IdentifyCredit, RawRole, Role};

#[derive(Debug, Clone)]
pub struct Arash;

impl Role for Arash {
    const TAG: &'static str = "arash";
    const RAW: RawRole = RawRole::Arash;

    fn map_identify<T>() -> crate::handlers::HandlerType<T>
    where
        T: 'static,
    {
        teloxide::dptree::filter_map(|ident: IdentifyCredit| {
            if ident == 106296897u64 {
                Some(Arash)
            } else {
                None
            }
        })
    }
}
