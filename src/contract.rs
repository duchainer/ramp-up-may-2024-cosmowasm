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
