use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{remove_schemas, write_api, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use dezswap::factory::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg, ConfigResponse, PairsResponse, NativeTokenDecimalsResponse};
use dezswap::asset::{AssetInfo, PairInfo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum LocalQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(PairInfo)]
    Pair {
        asset_infos: [AssetInfo; 2],
    },
    #[returns(PairsResponse)]
    Pairs {
        start_after: Option<[AssetInfo; 2]>,
        limit: Option<u32>,
    },
    #[returns(NativeTokenDecimalsResponse)]
    NativeTokenDecimals {
        denom: String,
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
