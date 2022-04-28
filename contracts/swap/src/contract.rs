#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::testing::MOCK_CONTRACT_ADDR;
use cosmwasm_std::{
    Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError,
    StdResult,
};

use cw2::set_contract_version;
use cw20::{Cw20Contract, Cw20ExecuteMsg, Cw20ReceiveMsg}; // ADDED

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

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
    
    // check if contract has enough mango to give
        // fail if contract doesn't have enough mango
    // contract keeps luna
    // contract gives mango



        // 이렇게 using exmaples - how did we do: mint, hooks, addr



    let state = STATE.load(deps.storage)?;
    if state.token_address != info.sender {
        return Err(ContractError::Unauthorized {});
    }




        // receive - 따로 transfer 할 필요? 아니면 그냥 contract 에다 보내진 양 storage 에다 저장?

    // let received_luna_amount: Cw20ReceiveMsg = received_msg.amount;

    // let this_contract_contract_helper = Cw20Contract(info.sender);
    
    // let state = STATE.load(deps.storage)?;

    // let contract_addr = state.token_address;

    // let msg = this_contract_contract_helper.call(Cw20ExecuteMsg::Transfer {
    //     recipient: contract_addr, // pot.target_addr.into_string()
    //     amount: received_luna_amount,
    // })?;
    // res = res.add_message(msg);



    


        // send to buyer wallet

    let token_amount_to_give = ???; // query Luna/Mango exchange rate from Oracle contract
    
    let recipient_addr = ???; // Luna sender address (is this info.sender?  그럼 위에서 why if state.token_address != info.sender)

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
