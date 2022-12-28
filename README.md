# DezSwap
[![dezswap on crates.io](https://img.shields.io/crates/v/dezswap.svg)](https://crates.io/crates/dezswap)
[![workflow](https://github.com/dezswap/dezswap-contracts/actions/workflows/tests.yml/badge.svg)](https://github.com/dezswap/dezswap-contracts/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/dezswap/dezswap-contracts/branch/main/graph/badge.svg?token=GQW0TPBBJH)](https://codecov.io/gh/dezswap/dezswap-contracts)

Uniswap-inspired automated market-maker (AMM) protocol powered by Smart Contracts on the [XPLA Chain](https://xpla.io/).

## Contracts

| Name                                               | Description                                  |
| -------------------------------------------------- | -------------------------------------------- |
| [`dezswap_factory`](contracts/dezswap_factory) |                                              |
| [`dezswap_pair`](contracts/dezswap_pair)       |                                              |
| [`dezswap_router`](contracts/dezswap_router)   |                                              |
| [`dezswap_token`](contracts/dezswap_token)     | CW20 (ERC20 equivalent) token implementation |

* dezswap_factory

   Mainnet: `xpla1j33xdql0h4kpgj2mhggy4vutw655u90z7nyj4afhxgj4v5urtadq44e3vd`

   Testnet: `xpla1j4kgjl6h4rt96uddtzdxdu39h0mhn4vrtydufdrk4uxxnrpsnw2qug2yx2`

* dezswap_pair

   Mainnet (CodeID): 28

   Testnet (CodeID): 263

* dezswap_token

   Mainnet (CodeID): 18

   Testnet (CodeID): 110

* dezswap_router

   Mainnet: `xpla1uv4dz7ngaqwymvxggrjp3rnz3gs33szwjsnrxqg0ylkykqf8r7ns9s3cg4`

   Testnet: `xpla1pr40depxf8w50y58swdyhc0s2yjptd2xtqgnyfvkz6k40ng53gqqnyftkm`

## Running this contract

You will need Rust 1.44.1+ with wasm32-unknown-unknown target installed.

You can run unit tests on this on each contracts directory via :

```
cargo unit-test
cargo integration-test
```

Once you are happy with the content, you can compile it to wasm on each contracts directory via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/cw1_subkeys.wasm .
ls -l cw1_subkeys.wasm
sha256sum cw1_subkeys.wasm
```

Or for a production-ready (compressed) build, run the following from the repository root:

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.8
```

The optimized contracts are generated in the artifacts/ directory.
