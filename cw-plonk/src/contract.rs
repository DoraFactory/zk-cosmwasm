use super::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use super::msg::{ProofResponse, ZkeysResponse};
use super::parser::{parse_proof, parse_vkey};
use super::state::{Config, ProofInfo, VkeyStr, CONFIG, PROVERINFO, PROVERLIST, ZKEYS};
use crate::coin_helpers::assert_sent_sufficient_coin;
use crate::state::ProofStr;
use crate::ContractError;
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult,
};

use pairing_ce::bn256::Bn256;

use bellman_ce::plonk::better_cs::cs::PlonkCsWidth4WithNextStepParams;
use bellman_ce::plonk::better_cs::verifier::verify as plonk_verify;
use bellman_ce::plonk::commitments::transcript::keccak_transcript::RollingKeccakTranscript;

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
            opening_at_z_omega_proof,
        ),
    }
}

pub fn execute_set_zkeys(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
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
        n,
        num_inputs,
        selector_commitments: selector_commitments
            .into_iter()
            .map(|x| hex::decode(x).unwrap())
            .collect(),
        next_step_selector_commitments: next_step_selector_commitments
            .into_iter()
            .map(|x| hex::decode(x).unwrap())
            .collect(),
        permutation_commitments: permutation_commitments
            .into_iter()
            .map(|x| hex::decode(x).unwrap())
            .collect(),
        non_residues,
        g2_elements: g2_elements
            .into_iter()
            .map(|x| hex::decode(x).unwrap())
            .collect(),
    };

    // jsut check the vkey is valid
    let _ = parse_vkey::<Bn256, PlonkCsWidth4WithNextStepParams>(vkeys.clone())?;

    ZKEYS.save(deps.storage, &info.sender, &vkeys)?;

    Ok(Response::default())
}

pub fn execute_publish_proof(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
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
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    assert_sent_sufficient_coin(&info.funds, config.proof_price)?;

    //  the issuer address is valid?
    let issuer = deps.api.addr_validate(&difficuty_issuer)?;

    if !(ZKEYS.may_load(deps.storage, &issuer)?).is_some() {
        // this issuer didn't public diffuculty problem
        return Err(ContractError::NonPublishDifficulty { difficuty_issuer });
    }

    let vkeys_str = ZKEYS.load(deps.storage, &issuer).unwrap();

    // verify the proof
    let proof_str = ProofStr {
        num_inputs,
        n,
        input_values,
        wire_commitments: wire_commitments
            .into_iter()
            .map(|x| hex::decode(x).unwrap())
            .collect(),
        grand_product_commitment: hex::decode(grand_product_commitment)
            .map_err(|_| ContractError::HexDecodingError {})?,
        quotient_poly_commitments: quotient_poly_commitments
            .into_iter()
            .map(|x| hex::decode(x).unwrap())
            .collect(),
        wire_values_at_z,
        wire_values_at_z_omega,
        grand_product_at_z_omega,
        quotient_polynomial_at_z,
        linearization_polynomial_at_z,
        permutation_polynomials_at_z,
        opening_at_z_proof: hex::decode(opening_at_z_proof)
            .map_err(|_| ContractError::HexDecodingError {})?,
        opening_at_z_omega_proof: hex::decode(opening_at_z_omega_proof)
            .map_err(|_| ContractError::HexDecodingError {})?,
    };

    let pof = parse_proof::<Bn256, PlonkCsWidth4WithNextStepParams>(proof_str.clone())?;
    let vkey = parse_vkey::<Bn256, PlonkCsWidth4WithNextStepParams>(vkeys_str)?;

    let ok =
        plonk_verify::<_, _, RollingKeccakTranscript<pairing_ce::bn256::Fr>>(&pof, &vkey, None)
            .map_err(|_| ContractError::SynthesisError {})?;

    if ok {
        let proof_info = ProofInfo {
            proof: proof_str,
            is_valid: ok,
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
        QueryMsg::Config {} => to_json_binary::<ConfigResponse>(&CONFIG.load(deps.storage)?.into()),
        QueryMsg::IssuerZkeys { address } => to_json_binary(&query_issuer_zkeys(deps, address)?),
        QueryMsg::ProofResult {
            issuer_address,
            prover_address,
        } => to_json_binary(&query_proof_result(deps, issuer_address, prover_address)?),
    }
}

fn query_issuer_zkeys(deps: Deps, address: String) -> StdResult<ZkeysResponse> {
    let issuer_addr = deps.api.addr_validate(&address)?;

    let vkeys = ZKEYS.load(deps.storage, &issuer_addr)?;
    Ok(ZkeysResponse {
        n: vkeys.n,
        num_inputs: vkeys.num_inputs,
        selector_commitments: vkeys
            .selector_commitments
            .into_iter()
            .map(|x| hex::encode(x))
            .collect(),
        next_step_selector_commitments: vkeys
            .next_step_selector_commitments
            .into_iter()
            .map(|x| hex::encode(x))
            .collect(),
        permutation_commitments: vkeys
            .permutation_commitments
            .into_iter()
            .map(|x| hex::encode(x))
            .collect(),
        non_residues: vkeys.non_residues,
        g2_elements: vkeys
            .g2_elements
            .into_iter()
            .map(|x| hex::encode(x))
            .collect(),
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
        num_inputs: proof_info.proof.num_inputs,
        n: proof_info.proof.n,
        input_values: proof_info.proof.input_values,
        wire_commitments: proof_info
            .proof
            .wire_commitments
            .into_iter()
            .map(|x| hex::encode(x))
            .collect(),
        grand_product_commitment: hex::encode(proof_info.proof.grand_product_commitment),
        quotient_poly_commitments: proof_info
            .proof
            .quotient_poly_commitments
            .into_iter()
            .map(|x| hex::encode(x))
            .collect(),
        wire_values_at_z: proof_info.proof.wire_values_at_z,
        wire_values_at_z_omega: proof_info.proof.wire_values_at_z_omega,
        grand_product_at_z_omega: proof_info.proof.grand_product_at_z_omega,
        quotient_polynomial_at_z: proof_info.proof.quotient_polynomial_at_z,
        linearization_polynomial_at_z: proof_info.proof.linearization_polynomial_at_z,
        permutation_polynomials_at_z: proof_info.proof.permutation_polynomials_at_z,
        opening_at_z_proof: hex::encode(proof_info.proof.opening_at_z_proof),
        opening_at_z_omega_proof: hex::encode(proof_info.proof.opening_at_z_omega_proof),
        is_valid: proof_info.is_valid,
    })
}
