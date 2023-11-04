use super::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use super::msg::{ProofResponse, ZkeysResponse};
use super::parser::{parse_proof, parse_vkey};
use super::state::{Config, ProofInfo, VkeyStr, ZkeysStr, CONFIG, PROVERINFO, PROVERLIST, ZKEYS};
use crate::coin_helpers::assert_sent_sufficient_coin;
use crate::state::ProofStr;
use crate::ContractError;
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use pairing_ce::bn256::Bn256;
// use ff_ce::PrimeField as Fr;
// use bellman_ce_verifier::{prepare_verifying_key, verify_proof};

// instantiate the contract
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let config = Config {
        zkeys_price: msg.set_zkeys_price,
        proof_price: msg.publish_proof_price,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Zkeys {
            public_signal,
            n,
            num_inputs,
            selector_commitments,
            next_step_selector_commitments,
            permutation_commitments,
            non_residues,
            g2_elements,
        } => execute_set_zkeys(
            deps,
            env,
            info,
            public_signal,
            n,
            num_inputs,
            selector_commitments,
            next_step_selector_commitments,
            permutation_commitments,
            non_residues,
            g2_elements,
        ),
        ExecuteMsg::Proof {
            difficuty_issuer,
            num_inputs,
            n,
            input_values,
            wire_commitments,
            grand_product_commitment,
            quotient_poly_commitments,
            wire_values_at_z,
            wire_values_at_z_omega,
            grand_product_at_z_omega,
            quotient_polynomial_at_z,
            linearization_polynomial_at_z,
            permutation_polynomials_at_z,
            opening_at_z_proof,
            opening_at_z_omega_proof,
        } => execute_publish_proof(
            deps, 
            env, 
            info, 
            difficuty_issuer,
            num_inputs,
            n,
            input_values,
            wire_commitments,
            grand_product_commitment,
            quotient_poly_commitments,
            wire_values_at_z,
            wire_values_at_z_omega,
            grand_product_at_z_omega,
            quotient_polynomial_at_z,
            linearization_polynomial_at_z,
            permutation_polynomials_at_z,
            opening_at_z_proof,
            opening_at_z_omega_proof
        ),
    }
}

pub fn execute_set_zkeys(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    public_signal: String,
    n: usize,
    num_inputs: usize,
    selector_commitments: Vec<String>,
    next_step_selector_commitments: Vec<String>,
    permutation_commitments: Vec<String>,
    non_residues: Vec<String>,
    g2_elements: Vec<String>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config.zkeys_price)?;
    // address
    // let key = info.sender.as_str().as_bytes();
    let vkeys = VkeyStr {
/*         alpha_1: hex::decode(vk_alpha1).map_err(|_| ContractError::HexDecodingError{})?,
        beta_2: hex::decode(vk_beta_2).map_err(|_| ContractError::HexDecodingError{})?,
        gamma_2: hex::decode(vk_gamma_2).map_err(|_| ContractError::HexDecodingError{})?,
        delta_2: hex::decode(vk_delta_2).map_err(|_| ContractError::HexDecodingError{})?,
        ic0: hex::decode(vk_ic0).map_err(|_| ContractError::HexDecodingError{})?,
        ic1: hex::decode(vk_ic1).map_err(|_| ContractError::HexDecodingError{})?, */

    };

    let _ = parse_vkey::<Bn256>(vkeys.clone())?;

    let zkeys = ZkeysStr {
        vkeys,
        public_signal,
    };

    ZKEYS.save(deps.storage, &info.sender, &zkeys)?;

    Ok(Response::default())
}

pub fn execute_publish_proof(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    difficuty_issuer: String,
    /*     proof_a: String,
    proof_b: String,
    proof_c: String, */
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
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config.proof_price)?;

    //  the issuer address is valid?
    let issuer = deps.api.addr_validate(&difficuty_issuer)?;

    if !(ZKEYS.may_load(deps.storage, &issuer)?).is_some() {
        // this issuer didn't public diffuculty problem
        return Err(ContractError::NonPublishDifficulty { difficuty_issuer });
    }

    let zkeys = ZKEYS.load(deps.storage, &issuer).unwrap();
    let vkeys_str = zkeys.vkeys;
    let public_inputs = zkeys.public_signal;

    // verify the proof
    let proof_str = ProofStr {
        num_inputs,
        n,
        input_values: input_values.into_iter().map(|x| )




/*         pi_a: hex::decode(proof_a).map_err(|_| ContractError::HexDecodingError{})?,
        pi_b: hex::decode(proof_b).map_err(|_| ContractError::HexDecodingError{})?,
        pi_c: hex::decode(proof_c).map_err(|_| ContractError::HexDecodingError{})?, */

    };

    let pof = parse_proof::<Bn256>(proof_str.clone())?;
    let vkey = parse_vkey::<Bn256>(vkeys_str)?;
    let pvk = prepare_verifying_key(&vkey);
    let is_passed = verify_proof(&pvk, &pof, &[Fr::from_str(&public_inputs).unwrap()])
        .map_err(|_| ContractError::ErrorVerificationKey {})?;

    if is_passed {
        let proof_info = ProofInfo {
            proof: proof_str,
            is_valid: is_passed,
        };
        // save the storage
        PROVERINFO.save(deps.storage, &info.sender, &proof_info)?;
        PROVERLIST.save(deps.storage, (&issuer, &info.sender), &proof_info)?;
    } else {
        return Err(ContractError::InvalidProof {});
    }

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary::<ConfigResponse>(&CONFIG.load(deps.storage)?.into()),
        QueryMsg::IssuerZkeys { address } => to_binary(&query_issuer_zkeys(deps, address)?),
        QueryMsg::ProofResult {
            issuer_address,
            prover_address,
        } => to_binary(&query_proof_result(deps, issuer_address, prover_address)?),
    }
}

/* fn query_issuer_zkeys(deps: Deps, address: String) -> StdResult<ZkeysResponse> {
    let issuer_addr = deps.api.addr_validate(&address)?;

    let zkeys = ZKEYS.load(deps.storage, &issuer_addr)?;
    Ok(ZkeysResponse {
        public_signal: zkeys.public_signal,
        vk_alpha1: hex::encode(zkeys.vkeys.alpha_1),
        vk_beta_2: hex::encode(zkeys.vkeys.beta_2),
        vk_gamma_2: hex::encode(zkeys.vkeys.gamma_2),
        vk_delta_2: hex::encode(zkeys.vkeys.delta_2),
        vk_ic0: hex::encode(zkeys.vkeys.ic0),
        vk_ic1: hex::encode(zkeys.vkeys.ic1),
    })
}

fn query_proof_result(
    deps: Deps,
    issuer_address: String,
    prover_address: String,
) -> StdResult<ProofResponse> {
    let issuer_addr = deps.api.addr_validate(&issuer_address)?;
    let prover_addr = deps.api.addr_validate(&prover_address)?;

    let proof_info = PROVERLIST.load(deps.storage, (&issuer_addr, &prover_addr))?;
    Ok(ProofResponse {
        proof_a: hex::encode(proof_info.proof.pi_a),
        proof_b: hex::encode(proof_info.proof.pi_b),
        proof_c: hex::encode(proof_info.proof.pi_c),
        is_valid: proof_info.is_valid,
    })
}
 */