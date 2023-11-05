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
    pub num_inputs: usize,
    pub n: usize,
    pub input_values: Vec<String>,
    pub wire_commitments: Vec<Vec<u8>>,
    pub grand_product_commitment: Vec<u8>,
    pub quotient_poly_commitments: Vec<Vec<u8>>,
    pub wire_values_at_z: Vec<String>,
    pub wire_values_at_z_omega: Vec<String>,
    pub grand_product_at_z_omega: String,
    pub quotient_polynomial_at_z: String,
    pub linearization_polynomial_at_z: String,
    pub permutation_polynomials_at_z: Vec<String>,
    pub opening_at_z_proof: Vec<u8>,
    pub opening_at_z_omega_proof: Vec<u8>,
}

#[cw_serde]
pub struct VkeyStr {
    pub n: usize,
    pub num_inputs: usize,
    pub selector_commitments: Vec<Vec<u8>>,
    pub next_step_selector_commitments: Vec<Vec<u8>>,
    pub permutation_commitments: Vec<Vec<u8>>,
    pub non_residues: Vec<String>,
    pub g2_elements: Vec<Vec<u8>>,
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