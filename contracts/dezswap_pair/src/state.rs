use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use dezswap::asset::PairInfoRaw;

pub const PAIR_INFO: Item<PairInfoRaw> = Item::new("pair_info");
pub const FACTORY_ADDR: Item<Addr> = Item::new("factory_addr");
