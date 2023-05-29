use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub zkeys_price: Option<Coin>,
    pub proof_price: Option<Coin>,
}

#[cw_serde]
pub struct ProofStr {
    pub pi_a: Vec<u8>,
    pub pi_b: Vec<u8>,
    pub pi_c: Vec<u8>,
}

#[cw_serde]
pub struct VkeyStr {
    pub alpha_1: Vec<u8>,
    pub beta_2: Vec<u8>,
    pub gamma_2: Vec<u8>,
    pub delta_2: Vec<u8>,
    pub ic0: Vec<u8>,
    pub ic1: Vec<u8>,
}

#[cw_serde]
pub struct ZkeysStr {
    pub vkeys: VkeyStr,
    pub public_signal: String,
}

#[cw_serde]
pub struct ProofInfo {
    pub proof: ProofStr,
    pub is_valid: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");
// issuer -> [ porver -> proofInfo ]
pub const PROVERINFO: Map<&Addr, ProofInfo> = Map::new("prover_info");
pub const PROVERLIST: Map<(&Addr, &Addr), ProofInfo> = Map::new("prover_list");
pub const ZKEYS: Map<&Addr, ZkeysStr> = Map::new("zkeys");