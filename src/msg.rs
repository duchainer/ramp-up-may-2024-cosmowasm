use cosmwasm_std::{Addr, Coin};
use serde::{Deserialize, Serialize};

use crate::state::SomeCoin;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct InitMsg {
    pub initial_value: Option<u64>,
    pub minimal_donation: Coin,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    ValueIncremented { value: u128 },
    DonationsSentToProject { project_address: Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ValueResp {
    pub value: u128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub struct DonationsTotalResp {
    pub net_amount: SomeCoins,
    pub raw_amount: SomeCoins,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecMsg {
    Donate { project_address: Addr },
    Withdraw {},
}
