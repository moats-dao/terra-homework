#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::testing::MOCK_CONTRACT_ADDR;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError,
    StdResult, WasmQuery, WasmMsg, BankQuery, Uint128, BankMsg, QueryRequest, Coin, to_binary, CosmosMsg
};

use cw2::set_contract_version;
use cw20::{Cw20Contract, Cw20ExecuteMsg, Cw20ReceiveMsg, Cw20QueryMsg}; // ADDED

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
        ExecuteMsg::Buy (cw20_msg) => try_buy(_deps, _env, _info, cw20_msg),
        ExecuteMsg::Withdraw { amount } => try_withdraw(),
    }

    Err(ContractError::NotImplemented {})
}


pub fn try_buy(deps: DepsMut, env: Env, info: MessageInfo, received_msg: Cw20ReceiveMsg) {

    let oracle_contract_addr = "terra15wwpcv7ze99jk5vwru7ntjf9a77ydljuw727c5";

    let luna_per_mango_price = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart{ // price = Luna/Mango
        contract_addr: oracle_contract_addr, // contract to query the public API of
        msg: to_binary(&QueryMsg::QueryPrice {})?,
    }))?;

    let mut num_lunas = Uint128::zero();
    for coin in info.funds.iter() {
        if coin.denom == "uluna" {
            num_lunas = coin.amount;
        }
    }

    
    let num_mango_to_transfer = match num_lunas.checked_div(Uint128::from(luna_per_mango_price)) {
        Ok(n) => n,
        Err(_) => return Err(ContractError::NotImplemented {}),
    };


    // check Mango balance

    let dex_contract_addr = env.contract.address.clone();;

    let token_contract_addr = "terra10unsuxpm0g9yfy5f02s4ep7mwdkqdemzue6c9l"; // dried mango

    let token_balance: Uint128 = query_token_balance(deps, token_contract_addr, dex_contract_addr)?;

    // return Err if not enough Mango to send

    if token_balance < num_mango_to_transfer {
        return Err(ContractError::InvalidQuantity {});
    }



    // send Mango to buyer
    let transfer_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: state.token_address.to_string(),
        funds: vec![],
        msg: to_binary(&Cw20ExecuteMsg::Transfer {
            recipient: info.sender.into(),
            amount: num_mango_to_transfer,
        })?,
    });

    Ok(Response::new().add_attribute("method", "try_buy"))
}


pub fn query_balance(deps: Deps, account_addr: Addr, denom: String) -> StdResult<Uint256> {
    // load price form the oracle
    let balance: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: account_addr.to_string(),
        denom,
    }))?;
    Ok(balance.amount.amount.into())
}

pub fn query_token_balance(deps: Deps, contract_addr: Addr, account_addr: Addr) -> StdResult<Uint256> {
    // load balance form the token contract
    let token_balance: Uint128 = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: contract_addr.to_string(), // token address
        msg: to_binary(&Cw20QueryMsg::Balance {
            address: account_addr.to_string(),  // dex address
        })?, 
    })).unwrap_or_else(|_| Uint128::zero());

    Ok(balance.into())
}


pub fn try_withdraw(amount: i32) {

    let state: State = STATE.load(deps.storage)?;

    if state.owner != info.sender {
        return Err(ContractError::Unauthorized{});
    }


    if amount <= 0 {
        return Err(ContractError::InvalidQuantity{});
    }

    let contract_addr = env.contract.address.clone();

    let luna_balance = query_balance(deps, contract_addr, "uluna".to_string())?;

    if luna_balance <  Uint128::from(amount as u128) {
        return Err(ContractError::InvalidQuantity{});
    }

    // send to own wallet
    let msg = CosmosMsg::Bank(BankMsg::Send {
        to_address: state.owner.clone().into_string(), // info.sender.clone().into_string()
        amount: vec![
            Coin {
                denom: "uluna".to_string(),
                amount: balance,
            },
        ],
    });

    
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
