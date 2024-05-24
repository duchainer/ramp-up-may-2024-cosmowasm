use cosmwasm_std::{
    Deps, DepsMut, Empty, entry_point, Env, MessageInfo, QueryResponse, Response, StdResult,
    to_json_binary,
};

use crate::contract::exec;
use crate::msg::{ExecMsg, QueryMsg};
use crate::state::FEE_COLLECTOR;

mod contract;
pub mod msg;
mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    // DONATIONS.save(deps.storage, )?;
    FEE_COLLECTOR.save(deps.storage, &info.sender)?;
    Ok(Response::new())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecMsg) -> StdResult<Response> {
    match msg {
        ExecMsg::Donate { project_address } => exec::donate(deps.storage, &info, project_address),
        ExecMsg::Withdraw {} => exec::withdraw(&deps, env, &info),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    use contract::query;
    use msg::QueryMsg::ValueIncremented;

    match msg {
        ValueIncremented { value } => to_json_binary(&query::value_incremented(value)),
        QueryMsg::DonationsSentToProject { project_address } => to_json_binary(
            &query::donations_sent_to_project(deps.storage, project_address)?,
        ),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, coins, Empty, Uint128};
    use cw_multi_test::{App, BasicApp, Contract, ContractWrapper, Executor};
    use parameterized::parameterized;

    use crate::{execute, instantiate, query};
    use crate::msg::{DonationsTotalResp, ExecMsg, QueryMsg};

    #[track_caller]
    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }

    #[track_caller]
    fn instantiate_contract(app: &mut BasicApp, owner: Addr) -> Addr {
        let contract_id = app.store_code(counting_contract());

        app.instantiate_contract(
            contract_id,
            owner,
            &Empty {},
            &[],
            "Counting contract",
            None,
        )
        .unwrap()
    }

    #[parameterized(
    raw_donation = {
        10, 10_000, 100_000
    },
    fee_amount = {
        1, 500, 5_000
    } )]
    fn donate_some_cw20_tokens(raw_donation: u128, fee_amount: u128) {
        let net_amount = raw_donation - fee_amount;
        let project_address =
            Addr::unchecked("cosmwasm1fventeva948ue0fzhp6xselr522rnqwger9wg7r0g9f4jemsqh6s9fcs2v");

        let fee_collector_address =
            Addr::unchecked("cosmwasm19mfs8tl4s396u7vqw9rrnsmrrtca5r66p7v8jvwdxvjn3shcmllqupdgxu");
        let donor =
            Addr::unchecked("cosmwasm1uzyszmsnca8euusre35wuqj4el3hyj8jty84kwln7du5stwwxyns2z5hxp");
        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &donor, coins(raw_donation, "cw20"))
                .unwrap();
        });

        let contract_addr = instantiate_contract(&mut app, fee_collector_address.clone());
        app.execute_contract(
            donor.clone(),
            contract_addr.clone(),
            &ExecMsg::Donate {
                project_address: project_address.clone(),
            },
            &coins(raw_donation, "cw20"),
        )
        .expect("We should be able to donate ");

        assert_eq!(
            app.wrap()
                .query_balance(project_address.clone(), "cw20")
                .expect("We should have some cw20 tokens")
                .amount,
            Uint128::new(net_amount)
        );

        assert_eq!(
            app.wrap()
                .query_balance(fee_collector_address, "cw20")
                .expect("We should have some cw20 tokens")
                .amount,
            Uint128::new(fee_amount)
        );

        assert_eq!(
            DonationsTotalResp{
                net_amount,
                raw_amount: raw_donation
            },
            app.wrap()
                .query_wasm_smart::<DonationsTotalResp>(
                    contract_addr,
                    &QueryMsg::DonationsSentToProject { project_address },
                )
                .expect("We should be able to get a response from using DonationsSentToProject on the contract_addr ")
        )
    }
}
// Using :
// ```
//     let addrs: Vec<_> = (0..10)
//     .map(|_| {
//         let addr = (&mut app, Some(0), Some(Coin::new(10u128, "CAD")));
//         addr.to_string()
//     })
//     .collect();
// ```
// const SOME_WALLET_ADDR_AS_STRING: [&str; 10] = [
// "cosmwasm1wug8sewp6cedgkmrmvhl3lf3tulagm9hnvy8p0rppz9yjw0g4wtqlrtkzd",
// "cosmwasm1qg5ega6dykkxc307y25pecuufrjkxkaggkkxh7nad0vhyhtuhw3sgetes3",
// "cosmwasm1zwv6feuzhy6a9wekh96cd57lsarmqlwxdypdsplw6zhfncqw6ftqnzgsl6",
// "cosmwasm1436kxs0w2es6xlqpp9rd35e3d0cjnw4sv8j3a7483sgks29jqwgsy4c2hs",
// "cosmwasm1mf6ptkssddfmxvhdx0ech0k03ktp6kf9yk59renau2gvht3nq2gq7z7vxe",
// "cosmwasm1wn625s4jcmvk0szpl85rj5azkfc6suyvf75q6vrddscjdphtve8suv7azv",
// "cosmwasm1tqwwyth34550lg2437m05mjnjp8w7h5ka7m70jtzpxn4uh2ktsmqruaxmk",
// "cosmwasm1gurgpv8savnfw66lckwzn4zk7fp394lpe667dhu7aw48u40lj6jsgzl0mg",
// "cosmwasm1999u8suptza3rtxwk7lspve02m406xe7l622erg3np3aq05gawxs4j90lk",
// USED "cosmwasm1fventeva948ue0fzhp6xselr522rnqwger9wg7r0g9f4jemsqh6s9fcs2v",
// ];
