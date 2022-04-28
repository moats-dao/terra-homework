use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub token_address: Addr,
}

pub const STATE: Item<State> = Item::new("state");




        // 아래는 별도로 추가

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractBalance {
    pub token_balance: Uint128,
    pub luna_balance: Uint128,
}

pub const DEX: Item<ContractBalance> = Item::new("dex");
