#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::testing::MOCK_CONTRACT_ADDR;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError,
    StdResult, WasmQuery,
};

use cw2::set_contract_version;
use cw20::{Cw20Contract, Cw20ExecuteMsg, Cw20ReceiveMsg}; // ADDED

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{STATE, DEX};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;


    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match _msg {
        ExecuteMsg::Buy (cw20_msg) => try_buy(_deps, _info, cw20_msg),
        ExecuteMsg::Withdraw { amount } => try_withdraw(),
    }

    Err(ContractError::NotImplemented {})
}


pub fn try_buy(deps: DepsMut, info: MessageInfo, received_msg: Cw20ReceiveMsg) {

    // query current luna/mango price

    let oracle_contract_addr_as_string = "";

    let price = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart{
        contract_addr: oracle_contract_addr_as_string, // the contract of which we want to query the public API of
        msg: to_binary(&QueryMsg::QueryPrice {})?,
    }))?;

    
    // check if contract has enough mango to give
        // fail if contract doesn't have enough mango
    // contract keeps luna
    // contract gives mango

    

    let mut num_lunas = Uint128::zero();

    for coin in info.funds.iter() {
        if coin.denom == "uluna" {
            num_lunas = coin.amount;
        }
    }







    let dex_contract_addr = "";

    let token_contract_addr = "";

    let balance: Uint128 = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: token_contract_addr, ???
        msg: to_binary(&Cw20QueryMsg::Balance {address: dex_contract_addr.to_string(), 
        })?, 
    })).unwrap_or_else(|_| Uint128::zero());






    let balance_response: BalanceResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: String::from(oracle_contract),
        msg: to_binary(&cw20::Cw20QueryMsg::Balance {
            address: oracle_contract.to_string(),
        })?,
    }))?;



    pub fn query_boost_amount(
        querier: &QuerierWrapper,
        boost_contract: &Addr,
        address: &Addr,
    ) -> StdResult<Uint128> {
        

        let balance_response: BalanceResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: String::from(oracle_contract),
            msg: to_binary(&cw20::Cw20QueryMsg::Balance {
                address: oracle_contract.to_string(),
            })?,
        }))?;
    
        Ok(res.total_boost)
    }












    let get_balance_msg = cw20::Cw20QueryMsg::Balance { address: String::from("") };

















    let state = STATE.load(deps.storage)?;
    if state.token_address != info.sender {
        return Err(ContractError::Unauthorized {});
    }







        // receive - 따로 transfer 할 필요? 아니면 그냥 contract 에다 보내진 양 storage 에다 저장?

    let received_luna_amount: Cw20ReceiveMsg = received_msg.amount;    

    let dex = DEX.load(deps.storage)?;

    dex.luna_balance += received_luna_amount;

    DEX.save(deps.storage, &dex)?;


    let contract_addr = state.token_address;

    let msg = this_contract_contract_helper.call(Cw20ExecuteMsg::Transfer {
        recipient: contract_addr, // pot.target_addr.into_string()
        amount: received_luna_amount,
    })?;
    res = res.add_message(msg);


    
    


        // send to buyer wallet

    let token_amount_to_give = ???; // query Luna/Mango exchange rate from Oracle contract

    dex.token_balance -= token_amount_to_give;

    DEX.save(deps.storage, &dex)?;
    
    let recipient_addr = received_msg.sender? ???; // Luna sender address (is this info.sender?  그럼 위에서 why if state.token_address != info.sender)     received_msg.sender?

    let recipient_contract_helper = Cw20Contract(info.sender); // here    info.sender    should be equal to token_address (위에서 확인)

    recipient_contract_helper.call(Cw20ExecuteMsg::Transfer {
        recipient: recipient_addr,
        amount: token_amount_to_give
    })?;





    // let msg = recipient_contract_helper.call(Cw20ExecuteMsg::Transfer {
    //     recipient: recipient_addr, // pot.target_addr.into_string()
    //     amount: token_amount_to_give
    // })?;
    // res = res.add_message(msg);






    

    Ok(Response::new().add_attribute("method", "try_buy"))
}


pub fn query_token_balance(
    deps: Deps,
    contract_addr: Addr,
    account_addr: Addr,
) -> StdResult<Uint256> {
    // load balance form the token contract
    let balance: Uint128 = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: contract_addr.to_string(),
            msg: to_binary(&Cw20QueryMsg::Balance {
                address: account_addr.to_string(),
            })?,
        }))
        .unwrap_or_else(|_| Uint128::zero());

    Ok(balance.into())
}






pub fn try_withdraw() {

}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: Empty) -> StdResult<Response> {
    // TODO
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    // TODO
    Err(StdError::generic_err("Not implemented"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn proper_initialization() {

        //TODO
    }
}
