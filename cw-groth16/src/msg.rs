use crate::state::Config;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    pub set_zkeys_price: Option<Coin>,
    pub publish_proof_price: Option<Coin>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Zkeys {
        public_signal: Vec<u8>,
        vk_alpha1: Vec<u8>,
        vk_beta_2: Vec<u8>,
        vk_gamma_2: Vec<u8>,
        vk_delta_2: Vec<u8>,
        vk_ic0: Vec<u8>,
        vk_ic1: Vec<u8>
    },
    Proof {
        proof_a: Vec<u8>,
        proof_b: Vec<u8>,
        proof_c: Vec<u8>,
    },
}
