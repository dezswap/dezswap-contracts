use std::env::current_dir;

use cosmwasm_schema::{export_schema, schema_for, write_api};

use dezswap::router::{Cw20HookMsg, ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema/raw");
    write_api! {
        instantiate: InstantiateMsg,
        query: QueryMsg,
        execute: ExecuteMsg,
        migrate: MigrateMsg,
    }
    export_schema(&schema_for!(Cw20HookMsg), &out_dir);
}
