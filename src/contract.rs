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
    use cosmwasm_std::{
        BankMsg, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Storage,
    };

    use crate::state::{COUNTER, MINIMAL_DONATION, OWNER};

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

    pub(crate) fn withdraw(deps: &DepsMut, env: Env, info: &MessageInfo) -> StdResult<Response> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(StdError::generic_err("Unauthorized"));
        }

        let balance = deps.querier.query_all_balances(env.contract.address)?;
        let sender = info.sender.to_string();
        let bank_msg = BankMsg::Send {
            to_address: sender.clone(),
            amount: balance,
        };
        let resp = Response::new()
            .add_message(bank_msg)
            .add_attribute("action", "withdraw")
            .add_attribute("sender", sender);
        Ok(resp)
    }
}
