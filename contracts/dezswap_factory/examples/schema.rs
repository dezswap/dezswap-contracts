use std::env::current_dir;

use cosmwasm_schema::write_api;

use dezswap::factory::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
        migrate: MigrateMsg,
    }
}
