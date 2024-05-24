use std::str::FromStr;

use cosmwasm_std::{Addr, Coin, Coins, Uint128};
use cw_storage_plus::{Item, Map};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct SomeCoins(pub Coins);

impl<'a> Deserialize<'a> for SomeCoins {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(SomeCoins(
            Coins::from_str(&s).expect("We should always have proper Coins here"),
        ))
    }
}

impl Serialize for SomeCoins {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.0.to_string();
        serializer.serialize_str(&s)
    }
}

//     fn add(self, other: Self) -> Self {
//         self.iter().map(|my_coin| other)
//     }
// }

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Donation {
    pub(crate) donor: Addr,
    pub(crate) raw_amount: SomeCoins,
}
impl Donation {
    pub fn net_amount(&self) -> SomeCoins {
        let mut coins = Coins::default();
        self.raw_amount.0.iter().for_each(|coin| {
            coins
                .add(Coin {
                    denom: coin.denom.clone(),
                    amount: Uint128::new(coin.amount.u128() - self.fee(coin.amount.u128())),
                })
                .expect("We should be able to add any valid coin to Coins")
        });
        SomeCoins(coins)
    }

    fn fee(&self, coin_amount: u128) -> u128 {
        if coin_amount < 10_000 {
            coin_amount / 10 // 10%
        } else {
            coin_amount / 20 // 5%
        }
    }
}

/// many Project to many { User to amount }
pub const DONATIONS: Map<&Addr, Vec<Donation>> = Map::new("donations");
pub const OWNER: Item<Addr> = Item::new("owner");
pub const FEE_COLLECTOR: Item<Addr> = Item::new("fee_collector");
