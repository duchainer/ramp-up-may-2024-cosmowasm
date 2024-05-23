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
    use cosmwasm_std::{Response, StdResult, Storage};

    use crate::state::COUNTER;

    pub fn poke(storage: &mut dyn Storage, sender: &str) -> StdResult<Response> {
        let new_value = COUNTER.load(storage)? + 1;
        COUNTER.save(storage, &new_value)?;

        let resp = Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", sender)
            .add_attribute("counter", new_value.to_string());

        Ok(resp)
    }
}
