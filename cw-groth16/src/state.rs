use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub zkeys_price: Option<Coin>,
    pub proof_price: Option<Coin>,
}