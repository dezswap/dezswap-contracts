use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{remove_schemas, write_api, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use dezswap::asset::{Asset, PairInfo};
use dezswap::pair::{
    Cw20HookMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, PoolResponse,
    ReverseSimulationResponse, SimulationResponse,
};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum LocalQueryMsg {
    #[returns(PairInfo)]
    Pair {},
    #[returns(PoolResponse)]
    Pool {},
    #[returns(SimulationResponse)]
    Simulation { offer_asset: Asset },
    #[returns(ReverseSimulationResponse)]
    ReverseSimulation { ask_asset: Asset },
}

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: LocalQueryMsg,
        migrate: MigrateMsg,
    }
}
