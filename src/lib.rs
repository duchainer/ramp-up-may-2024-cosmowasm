use cosmwasm_std::{
    Deps, DepsMut, Empty, entry_point, Env, MessageInfo, QueryResponse, Response, StdResult,
    to_json_binary,
};

use crate::msg::QueryMsg;

mod contract;
pub mod msg;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    use contract::query;
    use msg::QueryMsg::{Value, ValueIncremented};

    match msg {
        ValueIncremented { value } => to_json_binary(&query::value_incremented(value)),
        Value {} => to_json_binary(&query::value()),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, Empty};
    use cw_multi_test::{App, BasicApp, Contract, ContractWrapper, Executor};

    use crate::{execute, instantiate, query};
    use crate::msg::{QueryMsg, ValueResp};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }
    #[test]
    fn query_value() {
        let mut app = App::default();

        let contract_addr = instantiate_contract(&mut app);

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(ValueResp { value: 0 }, resp)
    }

    #[test]
    fn query_value_incremented() {
        let mut app = App::default();

        let contract_addr = instantiate_contract(&mut app);

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::ValueIncremented { value: 0 })
            .unwrap();

        assert_eq!(ValueResp { value: 1 }, resp)
    }

    fn instantiate_contract(app: &mut BasicApp) -> Addr {
        let contract_id = app.store_code(counting_contract());

        app.instantiate_contract(
            contract_id,
            Addr::unchecked("sender"),
            &Empty {},
            &[],
            "Counting contract",
            None,
        )
        .unwrap()
    }
}
