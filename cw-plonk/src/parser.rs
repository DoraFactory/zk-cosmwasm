use super::error::ContractError;
use crate::state::{ProofStr, VkeyStr};
use cosmwasm_std::ensure;
use ff_ce::from_hex;
use pairing_ce::bn256::{G1Affine, G1Uncompressed, G2Affine, G2Uncompressed};
use pairing_ce::{CurveAffine, EncodedPoint, Engine};
use bellman_ce::plonk::better_cs::keys::{ Proof, VerificationKey };
use bellman_ce::plonk::better_cs::cs::PlonkConstraintSystemParams;
use core::num;
use std::marker::PhantomData;

/// convert the proof into the Affine/Fr type, which will be used to verify
pub fn parse_proof<E, P>(pof: ProofStr) -> Result<Proof<E, P>, ContractError>
where
    E: Engine<G1Affine = G1Affine, G2Affine = G2Affine>,
    P: PlonkConstraintSystemParams<E>,
{
    let num_inputs = pof.num_inputs;
    let n = pof.n;
    // String -> Fr
    let input_values = pof.input_values;
    // Vec<u8> -> Uncompressed -> G1Affine
    let wire_commitments =  pof.wire_commitments;
    // Vec<u8> -> Uncompressed -> G1Affine
    let grand_product_commitment = pof.grand_product_commitment;
    // Vec<u8> -> Uncompressed -> G1Affine
    let quotient_poly_commitments = pof.quotient_poly_commitments;

    // String -> Fr
    let wire_values_at_z = pof.wire_values_at_z;
    // String -> Fr
    let wire_values_at_z_omega = pof.wire_values_at_z_omega;
    // String -> Fr
    let grand_product_at_z_omega = pof.grand_product_at_z_omega;
    // String -> Fr
    let quotient_polynomial_at_z = pof.quotient_polynomial_at_z;
    // String -> Fr
    let linearization_polynomial_at_z = pof.linearization_polynomial_at_z;
    // String -> Fr
    let permutation_polynomials_at_z = pof.permutation_polynomials_at_z;
    // Vec<u8> -> Uncompressed -> G1Affine
    let opening_at_z_proof = pof.opening_at_z_proof;
    // Vec<u8> -> Uncompressed -> G1Affine
    let opening_at_z_omega_proof = pof.opening_at_z_omega_proof;

    // ensure the format of proof is correct!
    ensure!(wire_commitments.iter().all(|inner_vec| inner_vec.len() == 64), ContractError::ErrorProof {});
    ensure!(grand_product_commitment.len() == 64, ContractError::ErrorProof {});
    ensure!(quotient_poly_commitments.iter().all(|inner_vec| inner_vec.len() == 64), ContractError::ErrorProof {});
    ensure!(opening_at_z_proof.len() == 64, ContractError::ErrorProof {});
    ensure!(opening_at_z_omega_proof.len() == 64, ContractError::ErrorProof {});


    // start transform the type
    let mut wire_commitments_affine: Vec<E::G1Affine> = Vec::new();
    let mut grand_product_commitment_arr: [u8; 64] = [0; 64];
    let mut quotient_poly_commitments_affine: Vec<E::G1Affine> = Vec::new();
    let mut opening_at_z_proof_arr: [u8; 64] = [0; 64];
    let mut opening_at_z_omega_proof_arr: [u8; 64] = [0; 64];

    wire_commitments_affine = wire_commitments.into_iter().map(|inner_vec| {
        let mut array = [0; 64];
        array[..inner_vec.len()].copy_from_slice(&inner_vec[..]);
        return G1Uncompressed::from_fixed_bytes(array).into_affine().unwrap();
    }).collect();
    grand_product_commitment_arr[..grand_product_commitment.len()].copy_from_slice(&grand_product_commitment[..]);
    quotient_poly_commitments_affine = quotient_poly_commitments.into_iter().map(|inner_vec| {
        let mut array = [0; 64];
        array[..inner_vec.len()].copy_from_slice(&inner_vec[..]);
        return G1Uncompressed::from_fixed_bytes(array).into_affine().unwrap();
    }).collect();
    opening_at_z_proof_arr[..opening_at_z_proof.len()].copy_from_slice(&opening_at_z_proof[..]);
    opening_at_z_omega_proof_arr[..opening_at_z_omega_proof.len()].copy_from_slice(&opening_at_z_omega_proof[..]);

    let grand_product_commitment_affine = G1Uncompressed::from_fixed_bytes(grand_product_commitment_arr)
    .into_affine()
    .map_err(|_| ContractError::ErrorProof {})?;

    let opening_at_z_proof_affine = G1Uncompressed::from_fixed_bytes(opening_at_z_proof_arr)
    .into_affine()
    .map_err(|_| ContractError::ErrorProof {})?;

    let opening_at_z_omega_proof_affine = G1Uncompressed::from_fixed_bytes(opening_at_z_omega_proof_arr)
    .into_affine()
    .map_err(|_| ContractError::ErrorProof {})?;

    // transform end

    // return the proof
    let mut final_proof = Proof::empty();
    final_proof.num_inputs = num_inputs;
    final_proof.n = n;
    final_proof.input_values = input_values.into_iter().map(|x| from_hex(&x).unwrap()).collect();
    final_proof.wire_commitments = wire_commitments_affine;
    final_proof.grand_product_commitment = grand_product_commitment_affine;
    final_proof.quotient_poly_commitments = quotient_poly_commitments_affine;

    final_proof.wire_values_at_z = wire_values_at_z.into_iter().map(|x| from_hex(&x).unwrap()).collect();
    final_proof.wire_values_at_z_omega = wire_values_at_z_omega.into_iter().map(|x| from_hex(&x).unwrap()).collect();
    final_proof.grand_product_at_z_omega = from_hex(&grand_product_at_z_omega).unwrap();
    final_proof.quotient_polynomial_at_z = from_hex(&quotient_polynomial_at_z).unwrap();
    final_proof.linearization_polynomial_at_z = from_hex(&linearization_polynomial_at_z).unwrap();
    final_proof.permutation_polynomials_at_z = permutation_polynomials_at_z.into_iter().map(|x| from_hex(&x).unwrap()).collect();

    final_proof.opening_at_z_proof = opening_at_z_proof_affine;
    final_proof.opening_at_z_omega_proof = opening_at_z_omega_proof_affine;

    Ok(final_proof)
}

/// convert the verification key into the affine type, which will be used in verification
pub fn parse_vkey<E, P>(vk: VkeyStr) -> Result<VerificationKey<E, P>, ContractError>
where
    E: Engine<G1Affine = G1Affine, G2Affine = G2Affine>,
    P: PlonkConstraintSystemParams<E>,
{
    let n = vk.n;
    let num_inputs = vk.num_inputs;
    let selector_commitments = vk.selector_commitments;
    let next_step_selector_commitments = vk.next_step_selector_commitments;
    let permutation_commitments = vk.permutation_commitments;

    let non_residues = vk.non_residues;
    let g2_elements = vk.g2_elements;


    ensure!(selector_commitments.iter().all(|inner_vec| inner_vec.len() == 64), ContractError::ErrorVerificationKey {});
    ensure!(next_step_selector_commitments.iter().all(|inner_vec| inner_vec.len() == 64), ContractError::ErrorVerificationKey {});
    ensure!(permutation_commitments.iter().all(|inner_vec| inner_vec.len() == 64), ContractError::ErrorVerificationKey {});
    ensure!(g2_elements.iter().all(|inner_vec| inner_vec.len() == 128), ContractError::ErrorVerificationKey {});

    let mut selector_commitments_affine: Vec<E::G1Affine> = Vec::new();
    let mut next_step_selector_commitments_affine: Vec<E::G1Affine> = Vec::new();
    let mut permutation_commitments_affine: Vec<E::G1Affine> = Vec::new();
    let mut g2_elements_affine: Vec<E::G2Affine> = Vec::new();

    selector_commitments_affine = selector_commitments.into_iter().map(|inner_vec| {
        let mut array = [0; 64];
        array[..inner_vec.len()].copy_from_slice(&inner_vec[..]);
        return G1Uncompressed::from_fixed_bytes(array).into_affine().unwrap();
    }).collect();;

    next_step_selector_commitments_affine = next_step_selector_commitments.into_iter().map(|inner_vec| {
        let mut array = [0; 64];
        array[..inner_vec.len()].copy_from_slice(&inner_vec[..]);
        return G1Uncompressed::from_fixed_bytes(array).into_affine().unwrap();
    }).collect();;

    permutation_commitments_affine = permutation_commitments.into_iter().map(|inner_vec| {
        let mut array = [0; 64];
        array[..inner_vec.len()].copy_from_slice(&inner_vec[..]);
        return G1Uncompressed::from_fixed_bytes(array).into_affine().unwrap();
    }).collect();;

    g2_elements_affine = g2_elements.into_iter().map(|inner_vec| {
        let mut array = [0; 128];
        array[..inner_vec.len()].copy_from_slice(&inner_vec[..]);
        return G2Uncompressed::from_fixed_bytes(array).into_affine().unwrap();
    }).collect();;

    let mut g2_elements_affine_arr: [E::G2Affine; 2] = [E::G2Affine::zero(); 2];
    g2_elements_affine_arr[..g2_elements_affine.len()].copy_from_slice(&g2_elements_affine[..]);

    // return verification key
    Ok(VerificationKey {
        n,
        num_inputs,
        selector_commitments: selector_commitments_affine,
        next_step_selector_commitments: next_step_selector_commitments_affine,
        permutation_commitments: permutation_commitments_affine,
        non_residues: non_residues.into_iter().map(|x| from_hex(&x).unwrap()).collect(),
        g2_elements: g2_elements_affine_arr,
        _marker: std::marker::PhantomData,
    })
}
