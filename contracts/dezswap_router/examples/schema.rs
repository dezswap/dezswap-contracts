use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{remove_schemas, write_api, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Uint128;

use dezswap::router::{ConfigResponse, Cw20HookMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, SimulateSwapOperationsResponse, SwapOperation};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum LocalQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(SimulateSwapOperationsResponse)]
    SimulateSwapOperations {
        offer_amount: Uint128,
        operations: Vec<SwapOperation>,
    },
    #[returns(SimulateSwapOperationsResponse)]
    ReverseSimulateSwapOperations {
        ask_amount: Uint128,
        operations: Vec<SwapOperation>,
    },
}

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: LocalQueryMsg,
        migrate: MigrateMsg,
    }
}
