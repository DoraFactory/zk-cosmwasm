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
        permutation_polynomials_at_z: Vec<String>,
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
    pub num_inputs: usize,
    pub n: usize,
    pub input_values: Vec<String>,
    pub wire_commitments: Vec<String>,
    pub grand_product_commitment: String,
    pub quotient_poly_commitments: Vec<String>,
    pub wire_values_at_z: Vec<String>,
    pub wire_values_at_z_omega: Vec<String>,
    pub grand_product_at_z_omega: String,
    pub quotient_polynomial_at_z: String,
    pub linearization_polynomial_at_z: String,
    pub permutation_polynomials_at_z: Vec<String>,
    pub opening_at_z_proof: String,
    pub opening_at_z_omega_proof: String,
    pub is_valid: bool,
}

#[cw_serde]
pub struct ZkeysResponse {
    pub public_signal: String,
    pub n: usize,
    pub num_inputs: usize,
    pub selector_commitments: Vec<String>,
    pub next_step_selector_commitments: Vec<String>,
    pub permutation_commitments: Vec<String>,
    pub non_residues: Vec<String>,
    pub g2_elements: Vec<String>,
}