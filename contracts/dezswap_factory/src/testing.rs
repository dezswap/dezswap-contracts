#![allow(deprecated)]

use crate::contract::{execute, instantiate, query, reply};
use dezswap::mock_querier::{mock_dependencies, WasmMockQuerier};

use crate::state::{pair_key, TmpPairInfo};

use cosmwasm_std::testing::{message_info, mock_env, MockApi, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{
    attr, coin, coins, from_json, to_json_binary, Binary, CosmosMsg, HexBinary, OwnedDeps, Reply,
    ReplyOn, Response, StdError, SubMsg, SubMsgResponse, SubMsgResult, Uint128, WasmMsg,
};
use cw20::Cw20ExecuteMsg;
use dezswap::asset::{Asset, AssetInfo, PairInfo};
use dezswap::factory::{
    ConfigResponse, ExecuteMsg, InstantiateMsg, NativeTokenDecimalsResponse, QueryMsg,
};
use dezswap::pair::{
    ExecuteMsg as PairExecuteMsg, InstantiateMsg as PairInstantiateMsg,
    MigrateMsg as PairMigrateMsg,
};

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        pair_code_id: 321u64,
        token_code_id: 123u64,
    };

    let info = message_info(&deps.api.addr_make("addr0000"), &[]);

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    let query_res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
    let config_res: ConfigResponse = from_json(&query_res).unwrap();
    assert_eq!(123u64, config_res.token_code_id);
    assert_eq!(321u64, config_res.pair_code_id);
    assert_eq!(deps.api.addr_make("addr0000").to_string(), config_res.owner);
}

#[test]
fn update_config() {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        pair_code_id: 321u64,
        token_code_id: 123u64,
    };

    let info = message_info(&deps.api.addr_make("addr0000"), &[]);

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

    // update owner
    let info = message_info(&deps.api.addr_make("addr0000"), &[]);
    let msg = ExecuteMsg::UpdateConfig {
        owner: Some(deps.api.addr_make("addr0001").to_string()),
        pair_code_id: None,
        token_code_id: None,
    };

    let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let query_res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
    let config_res: ConfigResponse = from_json(&query_res).unwrap();
    assert_eq!(123u64, config_res.token_code_id);
    assert_eq!(321u64, config_res.pair_code_id);
    assert_eq!(deps.api.addr_make("addr0001").to_string(), config_res.owner);

    // update left items
    let env = mock_env();
    let info = message_info(&deps.api.addr_make("addr0001"), &[]);
    let msg = ExecuteMsg::UpdateConfig {
        owner: None,
        pair_code_id: Some(100u64),
        token_code_id: Some(200u64),
    };

    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let query_res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
    let config_res: ConfigResponse = from_json(&query_res).unwrap();
    assert_eq!(200u64, config_res.token_code_id);
    assert_eq!(100u64, config_res.pair_code_id);
    assert_eq!(deps.api.addr_make("addr0001").to_string(), config_res.owner);

    // Unauthorized err
    let env = mock_env();
    let info = message_info(&deps.api.addr_make("addr0000"), &[]);
    let msg = ExecuteMsg::UpdateConfig {
        owner: None,
        pair_code_id: None,
        token_code_id: None,
    };

    let res = execute(deps.as_mut(), env, info, msg);
    match res {
        Err(StdError::GenericErr { msg, .. }) => assert_eq!(msg, "unauthorized"),
        _ => panic!("Must return unauthorized error"),
    }
}

fn init(
    mut deps: OwnedDeps<MockStorage, MockApi, WasmMockQuerier>,
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let msg = InstantiateMsg {
        pair_code_id: 321u64,
        token_code_id: 123u64,
    };

    let env = mock_env();
    let info = message_info(&deps.api.addr_make("addr0000"), &[]);

    deps.querier.with_token_balances(&[(
        &deps.api.addr_make("asset0001").to_string(),
        &[(
            &deps.api.addr_make("addr0000").to_string(),
            &Uint128::zero(),
        )],
    )]);
    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();

    deps
}

#[test]
fn create_pair() {
    let mut deps = mock_dependencies(&[coin(10u128, "uusd".to_string())]);
    deps = init(deps);
    deps.querier
        .with_dezswap_factory(&[], &[("uusd".to_string(), 6u8)]);
    let assets = [
        Asset {
            info: AssetInfo::NativeToken {
                denom: "uusd".to_string(),
            },
            amount: Uint128::zero(),
        },
        Asset {
            info: AssetInfo::Token {
                contract_addr: deps.api.addr_make("asset0001").to_string(),
            },
            amount: Uint128::zero(),
        },
    ];

    let msg = ExecuteMsg::CreatePair {
        assets: assets.clone(),
    };

    let env = mock_env();
    let info = message_info(&deps.api.addr_make("addr0000"), &[]);
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![
            attr("action", "create_pair"),
            attr("pair", format!("uusd-{}", deps.api.addr_make("asset0001")))
        ]
    );
    assert_eq!(
        res.messages,
        vec![SubMsg {
            id: 1,
            gas_limit: None,
            reply_on: ReplyOn::Success,
            msg: WasmMsg::Instantiate2 {
                msg: to_json_binary(&PairInstantiateMsg {
                    asset_infos: [
                        AssetInfo::NativeToken {
                            denom: "uusd".to_string(),
                        },
                        AssetInfo::Token {
                            contract_addr: deps.api.addr_make("asset0001").to_string(),
                        }
                    ],
                    token_code_id: 123u64,
                    asset_decimals: [6u8, 8u8]
                })
                .unwrap(),
                code_id: 321u64,
                funds: vec![],
                label: "pair".to_string(),
                admin: Some(MOCK_CONTRACT_ADDR.to_string()),
                salt: Binary::from(HexBinary::from_hex("4c3376b0233aaad57f1bc33febb92496dac198af502483aaaeec8d7f20ab162b").unwrap()),
            }
            .into(),
            payload: Binary::from(HexBinary::from_hex("7b22706169725f6b6579223a5b36342c3136312c36372c35382c3234362c39302c37372c3130362c36342c33382c3233362c33352c3138392c32322c3230332c33372c39372c3133382c3135362c3131352c3232362c3134312c3134302c3133352c37392c3133322c31332c3131372c3138322c38302c3230382c3232362c3131372c3131372c3131352c3130305d2c22617373657473223a5b7b22696e666f223a7b226e61746976655f746f6b656e223a7b2264656e6f6d223a2275757364227d7d2c22616d6f756e74223a2230227d2c7b22696e666f223a7b22746f6b656e223a7b22636f6e74726163745f61646472223a22514b46444f765a61545770414a75776a7652624c4a57474b6e4850696a5979485434514e64625a51304f493d227d7d2c22616d6f756e74223a2230227d5d2c2261737365745f646563696d616c73223a5b362c385d2c2273656e646572223a22636f736d7761736d3178756b756b6b3638746361793632396e6c686e687a6e64393039356573716c6e397976633070756e6c363435703736337a643573753777663836222c22706169725f636f6e74726163745f61646472223a22636f736d7761736d313963356c6a76347867677a3868726777326d6a6d7377727434686a6368747a7873327777726a7a6a61713033753779646e7071736e3530343878227d").unwrap())
        },]
    );
}

#[test]
fn create_pair_native_token_and_ibc_token() {
    let mut deps = mock_dependencies(&[
        coin(10u128, "uusd".to_string()),
        coin(10u128, "ibc/HASH".to_string()),
    ]);
    deps = init(deps);
    deps.querier.with_dezswap_factory(
        &[],
        &[("uusd".to_string(), 6u8), ("ibc/HASH".to_string(), 6u8)],
    );

    let assets = [
        Asset {
            info: AssetInfo::NativeToken {
                denom: "uusd".to_string(),
            },
            amount: Uint128::zero(),
        },
        Asset {
            info: AssetInfo::NativeToken {
                denom: "ibc/HASH".to_string(),
            },
            amount: Uint128::zero(),
        },
    ];

    let msg = ExecuteMsg::CreatePair {
        assets: assets.clone(),
    };

    let env = mock_env();
    let info = message_info(&deps.api.addr_make("addr0000"), &[]);
    let res = execute(deps.as_mut(), env, info, msg).unwrap();
    assert_eq!(
        res.attributes,
        vec![attr("action", "create_pair"), attr("pair", "uusd-ibc/HASH")]
    );
    assert_eq!(
        res.messages,
        vec![SubMsg {
            id: 1,
            gas_limit: None,
            reply_on: ReplyOn::Success,
            msg: WasmMsg::Instantiate2 {
                msg: to_json_binary(&PairInstantiateMsg {
                    asset_infos: [
                        AssetInfo::NativeToken {
                            denom: "uusd".to_string(),
                        },
                        AssetInfo::NativeToken {
                            denom: "ibc/HASH".to_string(),
                        }
                    ],
                    token_code_id: 123u64,
                    asset_decimals: [6u8, 6u8]
                })
                .unwrap(),
                code_id: 321u64,
                funds: vec![],
                label: "pair".to_string(),
                admin: Some(MOCK_CONTRACT_ADDR.to_string()),
                salt: Binary::from(HexBinary::from_hex("6d8038dd4733702c646c8432be0f7f89cc857e1144fb3ecd990bad4fcc1d82d4").unwrap()),
            }
            .into(),
            payload: Binary::from(HexBinary::from_hex("7b22706169725f6b6579223a5b3130352c39382c39392c34372c37322c36352c38332c37322c3131372c3131372c3131352c3130305d2c22617373657473223a5b7b22696e666f223a7b226e61746976655f746f6b656e223a7b2264656e6f6d223a2275757364227d7d2c22616d6f756e74223a2230227d2c7b22696e666f223a7b226e61746976655f746f6b656e223a7b2264656e6f6d223a226962632f48415348227d7d2c22616d6f756e74223a2230227d5d2c2261737365745f646563696d616c73223a5b362c365d2c2273656e646572223a22636f736d7761736d3178756b756b6b3638746361793632396e6c686e687a6e64393039356573716c6e397976633070756e6c363435703736337a643573753777663836222c22706169725f636f6e74726163745f61646472223a22636f736d7761736d31786d65676334366b367a637877726e356378653968396e687a636564373274766130646c7a6e7961703666396b3270796d3861737a373267306e227d").unwrap()),
        },]
    );
}

#[test]
fn fail_to_create_same_pair() {
    let mut deps = mock_dependencies(&[coin(10u128, "uusd".to_string())]);
    deps = init(deps);

    let assets = [
        Asset {
            info: AssetInfo::NativeToken {
                denom: "uusd".to_string(),
            },
            amount: Uint128::zero(),
        },
        Asset {
            info: AssetInfo::NativeToken {
                denom: "uusd".to_string(),
            },
            amount: Uint128::zero(),
        },
    ];

    let msg = ExecuteMsg::CreatePair { assets };

    let env = mock_env();
    let info = message_info(&deps.api.addr_make("addr0000"), &[]);
    execute(deps.as_mut(), env, info, msg).unwrap_err();
}

#[test]
fn fail_to_create_pair_with_unactive_denoms() {
    let mut deps = mock_dependencies(&[coin(10u128, "uusd".to_string())]);
    deps = init(deps);

    deps.querier
        .with_dezswap_factory(&[], &[("uusd".to_string(), 6u8)]);

    let assets = [
        Asset {
            info: AssetInfo::NativeToken {
                denom: "uxxx".to_string(),
            },
            amount: Uint128::zero(),
        },
        Asset {
            info: AssetInfo::NativeToken {
                denom: "uusd".to_string(),
            },
            amount: Uint128::zero(),
        },
    ];

    let msg = ExecuteMsg::CreatePair { assets };

    let env = mock_env();
    let info = message_info(&deps.api.addr_make("addr0000"), &[]);
    execute(deps.as_mut(), env, info, msg).unwrap_err();
}

#[test]
fn fail_to_create_pair_with_unknown_token() {
    let mut deps = mock_dependencies(&[coin(10u128, "uusd".to_string())]);

    let msg = InstantiateMsg {
        pair_code_id: 321u64,
        token_code_id: 123u64,
    };

    let env = mock_env();
    let info = message_info(&deps.api.addr_make("addr0000"), &[]);

    // we can just call .unwrap() to assert this was a success
    let _res = instantiate(deps.as_mut(), env, info, msg).unwrap();
    deps.querier
        .with_dezswap_factory(&[], &[("uluna".to_string(), 6u8)]);

    let assets = [
        Asset {
            info: AssetInfo::NativeToken {
                denom: "uluna".to_string(),
            },
            amount: Uint128::zero(),
        },
        Asset {
            info: AssetInfo::Token {
                contract_addr: deps.api.addr_make("xxx").to_string(),
            },
            amount: Uint128::zero(),
        },
    ];

    let msg = ExecuteMsg::CreatePair { assets };

    let env = mock_env();
    let info = message_info(&deps.api.addr_make("addr0000"), &[]);
    execute(deps.as_mut(), env, info, msg).unwrap_err();
}

#[test]
fn reply_only_create_pair() {
    let mut deps = mock_dependencies(&[]);

    deps.querier.with_token_balances(&[(
        &MOCK_CONTRACT_ADDR.to_string(),
        &[
            (
                &deps.api.addr_make("asset0000").to_string(),
                &Uint128::from(100u128),
            ),
            (
                &deps.api.addr_make("asset0001").to_string(),
                &Uint128::from(100u128),
            ),
        ],
    )]);

    let assets = [
        Asset {
            info: AssetInfo::Token {
                contract_addr: deps.api.addr_make("asset0000").to_string(),
            },
            amount: Uint128::zero(),
        },
        Asset {
            info: AssetInfo::Token {
                contract_addr: deps.api.addr_make("asset0001").to_string(),
            },
            amount: Uint128::zero(),
        },
    ];

    let raw_assets = [
        assets[0].to_raw(deps.as_ref().api).unwrap(),
        assets[1].to_raw(deps.as_ref().api).unwrap(),
    ];

    let raw_infos = [
        assets[0].info.to_raw(deps.as_ref().api).unwrap(),
        assets[1].info.to_raw(deps.as_ref().api).unwrap(),
    ];

    let pair_key = pair_key(&raw_infos);

    let reply_msg = Reply {
        id: 1,
        result: SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            msg_responses: vec![],
            #[allow(deprecated)]
            data: None,
        }),
        gas_used: 0,
        payload: to_json_binary(&TmpPairInfo {
            assets: raw_assets,
            pair_key,
            sender: deps.api.addr_make("addr0000"),
            asset_decimals: [8u8, 8u8],
            pair_contract_addr: deps.api.addr_make("0000"),
        })
        .unwrap(),
    };

    let asset_infos = [
        AssetInfo::Token {
            contract_addr: deps.api.addr_make("asset0000").to_string(),
        },
        AssetInfo::Token {
            contract_addr: deps.api.addr_make("asset0001").to_string(),
        },
    ];

    // register dezswap pair querier
    deps.querier.with_dezswap_factory(
        &[(
            &deps.api.addr_make("0000").to_string(),
            &PairInfo {
                asset_infos,
                contract_addr: deps.api.addr_make("0000").to_string(),
                liquidity_token: deps.api.addr_make("liquidity0000").to_string(),
                asset_decimals: [8u8, 8u8],
            },
        )],
        &[],
    );

    let res = reply(deps.as_mut(), mock_env(), reply_msg).unwrap();

    assert_eq!(res.messages.len(), 0);
    assert_eq!(
        res.attributes[0],
        attr("pair_contract_addr", deps.api.addr_make("0000").to_string())
    );
    assert_eq!(
        res.attributes[1],
        attr(
            "liquidity_token_addr",
            deps.api.addr_make("liquidity0000").to_string()
        )
    );
}

#[test]
fn reply_create_pair_with_provide() {
    let mut deps = mock_dependencies(&[]);

    deps.querier
        .with_balance(&[(&MOCK_CONTRACT_ADDR.to_string(), coins(100u128, "axpla"))]);

    deps.querier.with_token_balances(&[(
        &deps.api.addr_make("pair0000").to_string(),
        &[(
            &deps.api.addr_make("asset0000").to_string(),
            &Uint128::from(100u128),
        )],
    )]);

    let assets = [
        Asset {
            info: AssetInfo::NativeToken {
                denom: "axpla".to_string(),
            },
            amount: Uint128::from(100u128),
        },
        Asset {
            info: AssetInfo::Token {
                contract_addr: deps.api.addr_make("asset0000").to_string(),
            },
            amount: Uint128::from(100u128),
        },
    ];

    let raw_assets = [
        assets[0].to_raw(deps.as_ref().api).unwrap(),
        assets[1].to_raw(deps.as_ref().api).unwrap(),
    ];

    let raw_infos = [
        assets[0].info.to_raw(deps.as_ref().api).unwrap(),
        assets[1].info.to_raw(deps.as_ref().api).unwrap(),
    ];

    let pair_key = pair_key(&raw_infos);

    let reply_msg = Reply {
        id: 1,
        result: SubMsgResult::Ok(SubMsgResponse {
            events: vec![],
            msg_responses: vec![],
            #[allow(deprecated)]
            data: None,
        }),
        gas_used: 0,
        payload: to_json_binary(&TmpPairInfo {
            assets: raw_assets,
            pair_key,
            sender: deps.api.addr_make("addr0000"),
            asset_decimals: [18u8, 8u8],
            pair_contract_addr: deps.api.addr_make("pair0000"),
        })
        .unwrap(),
    };

    let asset_infos = [
        AssetInfo::NativeToken {
            denom: "axpla".to_string(),
        },
        AssetInfo::Token {
            contract_addr: deps.api.addr_make("asset0000").to_string(),
        },
    ];

    // register dezswap pair querier
    deps.querier.with_dezswap_factory(
        &[(
            &deps.api.addr_make("pair0000").to_string(),
            &PairInfo {
                asset_infos,
                contract_addr: deps.api.addr_make("pair0000").to_string(),
                liquidity_token: deps.api.addr_make("liquidity0000").to_string(),
                asset_decimals: [18u8, 8u8],
            },
        )],
        &[("axpla".to_string(), 18u8)],
    );

    let res = reply(deps.as_mut(), mock_env(), reply_msg).unwrap();

    assert_eq!(res.messages.len(), 3);
    assert_eq!(
        res.messages[0],
        SubMsg {
            id: 0,
            msg: CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: deps.api.addr_make("asset0000").to_string(),
                msg: to_json_binary(&Cw20ExecuteMsg::IncreaseAllowance {
                    spender: deps.api.addr_make("pair0000").to_string(),
                    amount: Uint128::from(100u128),
                    expires: None,
                })
                .unwrap(),
                funds: vec![],
            }),
            gas_limit: None,
            reply_on: ReplyOn::Never,
            payload: cosmwasm_std::Binary::default(),
        }
    );
    assert_eq!(
        res.messages[1],
        SubMsg {
            id: 0,
            msg: CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: deps.api.addr_make("asset0000").to_string(),
                msg: to_json_binary(&Cw20ExecuteMsg::TransferFrom {
                    owner: deps.api.addr_make("addr0000").to_string(),
                    amount: Uint128::from(100u128),
                    recipient: MOCK_CONTRACT_ADDR.to_string(),
                })
                .unwrap(),
                funds: vec![],
            }),
            gas_limit: None,
            reply_on: ReplyOn::Never,
            payload: cosmwasm_std::Binary::default(),
        }
    );
    assert_eq!(
        res.messages[2],
        SubMsg {
            id: 0,
            msg: CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: deps.api.addr_make("pair0000").to_string(),
                msg: to_json_binary(&PairExecuteMsg::ProvideLiquidity {
                    assets,
                    receiver: Some(deps.api.addr_make("addr0000").to_string()),
                    deadline: None,
                    slippage_tolerance: None,
                })
                .unwrap(),
                funds: coins(100u128, "axpla".to_string()),
            }),
            gas_limit: None,
            reply_on: ReplyOn::Never,
            payload: cosmwasm_std::Binary::default(),
        }
    );
    assert_eq!(
        res.attributes[0],
        attr(
            "pair_contract_addr",
            deps.api.addr_make("pair0000").to_string()
        )
    );
    assert_eq!(
        res.attributes[1],
        attr(
            "liquidity_token_addr",
            deps.api.addr_make("liquidity0000").to_string()
        )
    );
}

#[test]
fn failed_reply_with_unknown_id() {
    let mut deps = mock_dependencies(&[]);

    let res = reply(
        deps.as_mut(),
        mock_env(),
        Reply {
            id: 9,
            result: SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                msg_responses: vec![],
                #[allow(deprecated)]
                data: None,
            }),
            gas_used: 0,
            payload: cosmwasm_std::Binary::default(),
        },
    );

    assert_eq!(res, Err(StdError::generic_err("invalid reply msg")))
}

#[test]
fn normal_add_allow_native_token() {
    let mut deps = mock_dependencies(&[coin(1u128, "uluna".to_string())]);
    deps = init(deps);

    let msg = ExecuteMsg::AddNativeTokenDecimals {
        denom: "uluna".to_string(),
        decimals: 6u8,
    };

    let info = message_info(&deps.api.addr_make("addr0000"), &[]);

    assert_eq!(
        execute(deps.as_mut(), mock_env(), info, msg).unwrap(),
        Response::new().add_attributes(vec![
            ("action", "add_allow_native_token"),
            ("denom", "uluna"),
            ("decimals", "6"),
        ])
    );

    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::NativeTokenDecimals {
            denom: "uluna".to_string(),
        },
    )
    .unwrap();
    let res: NativeTokenDecimalsResponse = from_json(&res).unwrap();
    assert_eq!(6u8, res.decimals)
}

#[test]
fn failed_add_allow_native_token_with_non_admin() {
    let mut deps = mock_dependencies(&[coin(1u128, "uluna".to_string())]);
    deps = init(deps);

    let msg = ExecuteMsg::AddNativeTokenDecimals {
        denom: "uluna".to_string(),
        decimals: 6u8,
    };

    let info = message_info(&deps.api.addr_make("noadmin"), &[]);

    assert_eq!(
        execute(deps.as_mut(), mock_env(), info, msg),
        Err(StdError::generic_err("unauthorized"))
    );
}

#[test]
fn failed_add_allow_native_token_with_zero_factory_balance() {
    let mut deps = mock_dependencies(&[coin(0u128, "uluna".to_string())]);
    deps = init(deps);

    let msg = ExecuteMsg::AddNativeTokenDecimals {
        denom: "uluna".to_string(),
        decimals: 6u8,
    };

    let info = message_info(&deps.api.addr_make("addr0000"), &[]);

    assert_eq!(
        execute(deps.as_mut(), mock_env(), info, msg),
        Err(StdError::generic_err(
            "supply greater than zero is required by the factory for verification",
        ))
    );
}

#[test]
fn append_add_allow_native_token_with_already_exist_token() {
    let mut deps = mock_dependencies(&[coin(1u128, "uluna".to_string())]);
    deps = init(deps);

    let msg = ExecuteMsg::AddNativeTokenDecimals {
        denom: "uluna".to_string(),

        decimals: 6u8,
    };

    let info = message_info(&deps.api.addr_make("addr0000"), &[]);

    execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::NativeTokenDecimals {
            denom: "uluna".to_string(),
        },
    )
    .unwrap();
    let res: NativeTokenDecimalsResponse = from_json(&res).unwrap();
    assert_eq!(6u8, res.decimals);

    let msg = ExecuteMsg::AddNativeTokenDecimals {
        denom: "uluna".to_string(),
        decimals: 7u8,
    };

    execute(deps.as_mut(), mock_env(), info, msg).unwrap();

    let res = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::NativeTokenDecimals {
            denom: "uluna".to_string(),
        },
    )
    .unwrap();
    let res: NativeTokenDecimalsResponse = from_json(&res).unwrap();
    assert_eq!(7u8, res.decimals)
}

#[test]
fn normal_migrate_pair() {
    let mut deps = mock_dependencies(&[coin(1u128, "uluna".to_string())]);
    deps = init(deps);

    let msg = ExecuteMsg::MigratePair {
        code_id: Some(123u64),
        contract: deps.api.addr_make("contract0000").to_string(),
    };

    let info = message_info(&deps.api.addr_make("addr0000"), &[]);

    assert_eq!(
        execute(deps.as_mut(), mock_env(), info, msg).unwrap(),
        Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Migrate {
            contract_addr: deps.api.addr_make("contract0000").to_string(),
            new_code_id: 123u64,
            msg: to_json_binary(&PairMigrateMsg {}).unwrap(),
        })),
    );
}

#[test]
fn normal_migrate_pair_with_none_code_id_will_config_code_id() {
    let mut deps = mock_dependencies(&[coin(1u128, "uluna".to_string())]);
    deps = init(deps);

    let msg = ExecuteMsg::MigratePair {
        code_id: None,
        contract: deps.api.addr_make("contract0000").to_string(),
    };

    let info = message_info(&deps.api.addr_make("addr0000"), &[]);

    assert_eq!(
        execute(deps.as_mut(), mock_env(), info, msg).unwrap(),
        Response::new().add_message(CosmosMsg::Wasm(WasmMsg::Migrate {
            contract_addr: deps.api.addr_make("contract0000").to_string(),
            new_code_id: 321u64,
            msg: to_json_binary(&PairMigrateMsg {}).unwrap(),
        })),
    );
}

#[test]
fn failed_migrate_pair_with_no_admin() {
    let mut deps = mock_dependencies(&[coin(1u128, "uluna".to_string())]);
    deps = init(deps);

    let msg = ExecuteMsg::MigratePair {
        code_id: None,
        contract: deps.api.addr_make("contract0000").to_string(),
    };

    let info = message_info(&deps.api.addr_make("noadmin"), &[]);

    assert_eq!(
        execute(deps.as_mut(), mock_env(), info, msg),
        Err(StdError::generic_err("unauthorized")),
    );
}
