#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, from_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{STATE, State};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:oracle";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let state = State {
        owner: _info.sender.clone(),
        price: Uint128::from(_msg.price),
    };
    STATE.save(deps.storage, &state)?;

    // TODO: instantiate contract
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    //TODO: execute try_update_price
    match _msg {
        ExecuteMsg::UpdatePrice { price } => try_update_price(_deps, _info, price),
    }

    // Ok(Response::new())
}

fn try_update_price(deps: DepsMut, info: MessageInfo, price: Uint128) -> Result<Response, ContractError> {

    let current_price = Uint128::from(price); // Luna / Mango

    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.price = current_price;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_update_price"))

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    match _msg {
        QueryMsg::QueryPrice {} => to_binary(&query_price(_deps)?),
    }
    // Err(StdError::generic_err("not implemented"))
}

fn query_price(deps: Deps) -> StdResult<Uint128> {
    let state = STATE.load(deps.storage)?;
    Ok(state.price)
}



#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

            // instantiate
        let msg = InstantiateMsg { price: Uint128::from(17u128) };
        let info = mock_info("creator", &coins(1000, "earth"));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

            // instantiate test
        let msg = QueryMsg::QueryPrice {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let price: Uint128 = from_binary(&res).unwrap();
        assert_eq!(Uint128::from(17u128), price);
    }

    #[test]
    fn test_update_price() {

        let mut deps = mock_dependencies(&[]);

            // instantiate
        let msg = InstantiateMsg { price: Uint128::from(17u128) };
        let info = mock_info("creator", &coins(1000, "earth"));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

            // UpdatePrice
        let msg = ExecuteMsg::UpdatePrice { price: (Uint128::from(25u128)) };
        let info = mock_info("creator", &coins(1000, "earth")); // we redeclare info here since we moved it into instantiate() for _res
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

            // UpdatePrice test
        let res = query(deps.as_ref(), mock_env(), QueryMsg::QueryPrice {}).unwrap(); // as_ref, so, not as_mut        also,   .unwrap()    not safe to do in the code, but can be done in tests all you want
        let new_price: Uint128 = from_binary(&res).unwrap(); // change back to not binary, i.e. de serialize it    from_binary returns a Result
        assert_eq!(Uint128::from(25u128), new_price); // we grab the value associated with "creator" in our Map      and see if it's 1
    
    }
}
