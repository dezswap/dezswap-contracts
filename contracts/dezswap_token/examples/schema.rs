use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{remove_schemas, write_api, QueryResponses};
use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw20::{
    AllAccountsResponse, AllAllowancesResponse, AllowanceResponse, BalanceResponse,
    TokenInfoResponse,
};
use cw20_base::msg::ExecuteMsg;
use dezswap::token::InstantiateMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema, QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum LocalQueryMsg {
    #[returns(BalanceResponse)]
    Balance { address: String },
    #[returns(TokenInfoResponse)]
    TokenInfo {},
    #[returns(AllowanceResponse)]
    Allowance { owner: String, spender: String },
    #[returns(AllAllowancesResponse)]
    AllAllowances {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(AllAccountsResponse)]
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: LocalQueryMsg,
    }
}
