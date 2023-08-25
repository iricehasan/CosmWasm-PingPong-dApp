use cosmwasm_schema::cw_serde;
use cw_storage_plus::Item;

pub const PING_COUNT: Item<u64> = Item::new("ping_count");