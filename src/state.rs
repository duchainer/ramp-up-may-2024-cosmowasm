use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Donation {
    pub(crate) donor: Addr,
    pub(crate) amount: u128,
}

/// many Project to many { User to amount }
pub const DONATIONS: Map<&Addr, Vec<Donation>> = Map::new("donations");
pub const OWNER: Item<Addr> = Item::new("owner");
pub const FEE_COLLECTOR: Item<Addr> = Item::new("fee_collector");
