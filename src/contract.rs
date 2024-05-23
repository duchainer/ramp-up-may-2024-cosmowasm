pub mod query {
    use cosmwasm_std::{StdResult, Storage};

    use crate::msg::ValueResp;
    use crate::state::COUNTER;

    pub fn value(storage: &dyn Storage) -> StdResult<ValueResp> {
        let value = COUNTER.load(storage)?;
        Ok(ValueResp { value })
    }
    pub fn value_incremented(n: u64) -> ValueResp {
        ValueResp { value: n + 1 }
    }
}

pub mod exec {
    use cosmwasm_std::{MessageInfo, Response, StdResult, Storage};

    use crate::state::{COUNTER, MINIMAL_DONATION};

    pub fn donate(storage: &mut dyn Storage, info: &MessageInfo) -> StdResult<Response> {
        let mut new_value = COUNTER.load(storage)?;
        let minimal_donation = MINIMAL_DONATION.load(storage)?;

        if info.funds.iter().any(|coin| {
            coin.denom == minimal_donation.denom && coin.amount >= minimal_donation.amount
        }) {
            new_value += 1;
            COUNTER.save(storage, &new_value)?;
        }

        let resp = Response::new()
            .add_attribute("action", "donate")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", new_value.to_string());

        Ok(resp)
    }
}
