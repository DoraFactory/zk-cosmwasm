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
        public_signal: String,
        n: usize,
        num_inputs: usize,
        selector_commitments: Vec<String>,
        next_step_selector_commitments: Vec<String>,
        permutation_commitments: Vec<String>,
        non_residues: Vec<String>,
        g2_elements: Vec<String>,
    },
    Proof {
        difficuty_issuer: String,
/*         proof_a: String,
        proof_b: String,
        proof_c: String */
        num_inputs: usize,
        n: usize,
        input_values: Vec<String>,
        wire_commitments: Vec<String>,
        grand_product_commitment: String,
        quotient_poly_commitments: Vec<String>,
        wire_values_at_z: Vec<String>,
        wire_values_at_z_omega: Vec<String>,
        grand_product_at_z_omega: String,
        quotient_polynomial_at_z: String,
        linearization_polynomial_at_z: String,
        permutation_polynomials_at_z: String,
        opening_at_z_proof: String,
        opening_at_z_omega_proof: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(ZkeysResponse)]
    IssuerZkeys { address: String},
    #[returns(ProofResponse)]
    ProofResult {issuer_address: String, prover_address: String}
}

#[cw_serde]
pub struct ConfigResponse {
    pub zkeys_price: Option<Coin>,
    pub proof_price: Option<Coin>,
}

impl From<Config> for ConfigResponse {
    fn from(config: Config) -> ConfigResponse {
        ConfigResponse {
            zkeys_price: config.zkeys_price,
            proof_price: config.proof_price,
        }
    }
}


#[cw_serde]
pub struct ProofResponse {
/*     pub proof_a: String,
    pub proof_b: String,
    pub proof_c: String, */
    pub is_valid: bool,
}

#[cw_serde]
pub struct ZkeysResponse {
    pub public_signal: String,
/*     pub vk_alpha1: String,
    pub vk_beta_2: String,
    pub vk_gamma_2: String,
    pub vk_delta_2: String,
    pub vk_ic0: String,
    pub vk_ic1: String */
}