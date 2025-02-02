use cosmwasm_std::{
    Deps, DepsMut, entry_point, Env, MessageInfo, QueryResponse, Response, StdResult,
    to_json_binary,
};

use crate::contract::exec;
use crate::msg::{ExecMsg, InitMsg, QueryMsg};
use crate::state::{COUNTER, MINIMAL_DONATION, OWNER};

mod contract;
pub mod msg;
mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> StdResult<Response> {
    COUNTER.save(deps.storage, &msg.initial_value.unwrap_or(0))?;
    MINIMAL_DONATION.save(deps.storage, &msg.minimal_donation)?;
    OWNER.save(deps.storage, &info.sender)?;
    Ok(Response::new())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecMsg) -> StdResult<Response> {
    match msg {
        ExecMsg::Donate {} => exec::donate(deps.storage, &info),
        ExecMsg::Withdraw {} => exec::withdraw(&deps, env, &info),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    use contract::query;
    use msg::QueryMsg::{Value, ValueIncremented};

    match msg {
        ValueIncremented { value } => to_json_binary(&query::value_incremented(value)),
        Value {} => to_json_binary(&query::value(deps.storage)?),
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Addr, coin, Coin, coins, Empty};
    use cw_multi_test::{App, BasicApp, Contract, ContractWrapper, Executor};
    use proptest::proptest;

    use crate::{execute, instantiate, query};
    use crate::msg::{ExecMsg, InitMsg, QueryMsg, ValueResp};

    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        Box::new(contract)
    }
    #[test]
    fn query_value_with_default_initial_value() {
        let mut app = App::default();

        let contract_addr = instantiate_contract(&mut app, None, None);

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(ValueResp { value: 0 }, resp)
    }

    proptest! {
       #[test]
       fn query_value_with_given_initial_value(initial_value: u64) {
           let mut app = App::default();

           let contract_addr = instantiate_contract(&mut app, Some(initial_value), None);

           let resp: ValueResp = app
               .wrap()
               .query_wasm_smart(contract_addr, &QueryMsg::Value {})
               .unwrap();

           assert_eq!(ValueResp { value: initial_value }, resp)
       }
    }

    #[test]
    fn donate_under_minimal_donation() {
        let mut app = App::default();

        let contract_addr = instantiate_contract(&mut app, None, Some(Coin::new(10u128, "CAD")));

        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &[],
        )
        .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 0 });
    }

    #[test]
    fn donate_with_minimal_donation_amount() {
        let sender = Addr::unchecked("sender");
        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(20, "CAD"))
                .unwrap();
        });

        let contract_addr = instantiate_contract(&mut app, Some(0), Some(Coin::new(10u128, "CAD")));

        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &coins(10u128, "CAD"),
        )
        .expect("We should be able to donate ");

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 1 });

        app.execute_contract(
            sender,
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &[Coin::new(10u128, "CAD")],
        )
        .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr.clone(), &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 2 });
    }

    #[test]
    fn query_value_incremented() {
        let mut app = App::default();

        let contract_addr = instantiate_contract(&mut app, None, None);

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::ValueIncremented { value: 0 })
            .unwrap();

        assert_eq!(ValueResp { value: 1 }, resp)
    }
    #[test]
    fn withdraw() {
        // We try to use bech32 valid addresses, for working query_all_balances and query_balance, because of addr_canonicalize
        // Example of valid bech32 : "cosmwasm1mzdhwvvh22wrt07w59wxyd58822qavwkx5lcej7aqfkpqqlhaqfsgn6fq2"
        let owner =
            Addr::unchecked("cosmwasm19mfs8tl4s396u7vqw9rrnsmrrtca5r66p7v8jvwdxvjn3shcmllqupdgxu");
        let sender =
            Addr::unchecked("cosmwasm1uzyszmsnca8euusre35wuqj4el3hyj8jty84kwln7du5stwwxyns2z5hxp");

        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(10, "atom"))
                .unwrap();
        });

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                owner.clone(),
                &InitMsg {
                    initial_value: Some(0),
                    minimal_donation: coin(10, "atom"),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &coins(10, "atom"),
        )
        .unwrap();

        app.execute_contract(
            owner.clone(),
            contract_addr.clone(),
            &ExecMsg::Withdraw {},
            &[],
        )
        .unwrap();

        assert_eq!(
            app.wrap().query_all_balances(owner).unwrap(),
            coins(10, "atom")
        );
        assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
        assert_eq!(
            app.wrap().query_all_balances(contract_addr).unwrap(),
            vec![]
        );
    }

    fn instantiate_contract(
        app: &mut BasicApp,
        initial_value: Option<u64>,
        minimal_donation: Option<Coin>,
    ) -> Addr {
        let contract_id = app.store_code(counting_contract());

        app.instantiate_contract(
            contract_id,
            Addr::unchecked("sender"),
            &InitMsg {
                initial_value,
                minimal_donation: minimal_donation.unwrap_or(Coin::new(1u128, "CAD")),
            },
            &[],
            "Counting contract",
            None,
        )
        .unwrap()
    }
}
