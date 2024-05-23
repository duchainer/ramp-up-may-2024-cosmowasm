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
    use msg::QueryMsg::Value;

    match msg {
        Value { value } => to_json_binary(&query::value(value)),
    }
}
