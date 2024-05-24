pub mod query {
    use crate::msg::ValueResp;

    pub fn value_incremented(n: u64) -> ValueResp {
        ValueResp { value: n + 1 }
    }
}

pub mod exec {
    use cosmwasm_std::{
        Addr, BankMsg, coins, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Storage,
    };

    use crate::state::{Donation, DONATIONS, FEE_COLLECTOR, OWNER};

    pub fn donate(
        storage: &mut dyn Storage,
        info: &MessageInfo,
        project_address: Addr,
    ) -> StdResult<Response> {
        if !DONATIONS.has(storage, &project_address) {
            // return Err(StdError::not_found("Project_address not found"));
            DONATIONS.save(storage, &project_address, &Vec::new());
        }

        let received_funds = info
            .funds
            .iter()
            .filter(|coin| coin.denom == "cw20")
            .collect::<Vec<_>>();
        if received_funds.len() == 0 {
            return Err(StdError::generic_err("Only deals with cw20 tokens"));
        }

        if received_funds.len() > 1 {
            return Err(StdError::generic_err(
                "We should not get 2 elements with the cw20 denom",
            ));
        }
        let coin = received_funds
            .get(0)
            .expect("The vec should be of size 1. We checked");

        let mut old_value = DONATIONS
            .load(storage, &project_address)
            .expect("We already checked that the project_address exists");
        old_value.push(Donation {
            donor: info.sender.clone(),
            amount: coin.amount.into(),
        });
        DONATIONS
            .save(storage, &project_address, &old_value)
            .expect("We should be able to push a new donation");

        let fee_amount = coin.amount.u128() / 10;
        let net_balance = coin.amount.u128() - fee_amount;

        let project_bank_msg = BankMsg::Send {
            to_address: project_address.to_string(),
            amount: coins(net_balance, "cw20"),
        };

        let fee_bank_msg = BankMsg::Send {
            to_address: FEE_COLLECTOR
                .load(storage)
                .expect("We should have set the fee collector address in the instantiate()")
                .to_string(),
            amount: coins(fee_amount, "cw20"),
        };

        let resp = Response::new()
            .add_attribute("action", "donate")
            .add_attribute("sender", info.sender.as_str())
            .add_message(project_bank_msg)
            .add_attribute("to", project_address.as_str())
            .add_attribute("net_amount", net_balance.to_string())
            .add_message(fee_bank_msg)
            .add_attribute("fee_amount", fee_amount.to_string());

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
