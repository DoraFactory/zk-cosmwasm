use crate::state::{ProofStr, VkeyStr};
use bellman_verifier::{Proof, VerifyingKey};
use bls12_381::{G1Affine, G2Affine};
use pairing::Engine;
use super::error::ContractError;
use cosmwasm_std::ensure;

/// convert the proof into the affine type, which will be used to verify
pub fn parse_proof<E>(pof: ProofStr) -> Result<Proof<E>, ContractError> 
where
	E: Engine<G1Affine = G1Affine, G2Affine = G2Affine>,
{
	let pi_a = pof.pi_a;
	let pi_b = pof.pi_b;
	let pi_c = pof.pi_c;

	ensure!(pi_a.len() == 96, ContractError::ErrorProof{});
	ensure!(pi_b.len() == 192, ContractError::ErrorProof{});
	ensure!(pi_c.len() == 96, ContractError::ErrorProof{});

	let mut a_arr: [u8; 96] = [0; 96];
	let mut b_arr: [u8; 192] = [0; 192];
	let mut c_arr: [u8; 96] = [0; 96];

	a_arr[..pi_a.len()].copy_from_slice(&pi_a[..]);

	b_arr[..pi_b.len()].copy_from_slice(&pi_b[..]);

	c_arr[..pi_c.len()].copy_from_slice(&pi_c[..]);

	let pia_affine = G1Affine::from_uncompressed(&a_arr);
	let pib_affine = G2Affine::from_uncompressed(&b_arr);
	let pic_affine = G1Affine::from_uncompressed(&c_arr);

	ensure!(pia_affine.is_some().unwrap_u8()== 1, ContractError::ErrorProof{});
	ensure!(pib_affine.is_some().unwrap_u8()== 1, ContractError::ErrorProof{});
	ensure!(pic_affine.is_some().unwrap_u8()== 1, ContractError::ErrorProof{});

	Ok(Proof { a: pia_affine.unwrap(), b: pib_affine.unwrap(), c: pic_affine.unwrap() })
}


/// convert the verification key into the affine type, which will be used in verification
pub fn parse_vkey<E>(vk: VkeyStr) ->  Result<VerifyingKey<E>, ContractError>
where
	E: Engine<G1Affine = G1Affine, G2Affine = G2Affine>,
{
	let vk_alpha_1 = vk.alpha_1;
	let vk_beta_2 = vk.beta_2;
	let vk_gamma_2 = vk.gamma_2;
	let vk_delta_2 = vk.delta_2;
	let vk_ic0 = vk.ic0;
	let vk_ic1 = vk.ic1;

	ensure!(vk_alpha_1.len() == 96, ContractError::ErrorVerificationKey{});
	ensure!(vk_beta_2.len() == 192, ContractError::ErrorVerificationKey{});
	ensure!(vk_gamma_2.len() == 192, ContractError::ErrorVerificationKey{});
	ensure!(vk_delta_2.len() == 192, ContractError::ErrorVerificationKey{});
	ensure!(vk_ic0.len() == 96, ContractError::ErrorVerificationKey{});
	ensure!(vk_ic1.len() == 96, ContractError::ErrorVerificationKey{});

	let mut alpha1: [u8; 96] = [0; 96];
	let mut beta2: [u8; 192] = [0; 192];
	let mut gamma2: [u8; 192] = [0; 192];
	let mut delta2: [u8; 192] = [0; 192];
	let mut ic_0: [u8; 96] = [0; 96];
	let mut ic_1: [u8; 96] = [0; 96];
	let mut ic = Vec::new();

	alpha1[..vk_alpha_1.len()].copy_from_slice(&vk_alpha_1[..]);

	beta2[..vk_beta_2.len()].copy_from_slice(&vk_beta_2[..]);

	gamma2[..vk_gamma_2.len()].copy_from_slice(&vk_gamma_2[..]);

	delta2[..vk_delta_2.len()].copy_from_slice(&vk_delta_2[..]);

	ic_0[..vk_ic0.len()].copy_from_slice(&vk_ic0[..]);

	ic_1[..vk_ic1.len()].copy_from_slice(&vk_ic1[..]);

	let alpha1_affine = G1Affine::from_uncompressed(&alpha1);
	let beta2_affine = G2Affine::from_uncompressed(&beta2);
	let gamma2_affine = G2Affine::from_uncompressed(&gamma2);
	let delta2_affine = G2Affine::from_uncompressed(&delta2);
	let ic0_affine = G1Affine::from_uncompressed(&ic_0);
	let ic1_affine = G1Affine::from_uncompressed(&ic_1);

	// ensure the format of verification key is correct, otherwaise, it can not be converted into affine type
	ensure!(alpha1_affine.is_some().unwrap_u8() == 1, ContractError::ErrorVerificationKey{});
	ensure!(beta2_affine.is_some().unwrap_u8()== 1, ContractError::ErrorVerificationKey{});
	ensure!(gamma2_affine.is_some().unwrap_u8()== 1, ContractError::ErrorVerificationKey{});
	ensure!(delta2_affine.is_some().unwrap_u8()== 1, ContractError::ErrorVerificationKey{});
	ensure!(ic0_affine.is_some().unwrap_u8()== 1, ContractError::ErrorVerificationKey{});
	ensure!(ic1_affine.is_some().unwrap_u8()== 1, ContractError::ErrorVerificationKey{});

	ic.push(ic0_affine.unwrap());
	ic.push(ic1_affine.unwrap());

	// return verification key
	Ok(VerifyingKey {
		alpha_g1: alpha1_affine.unwrap(),
		beta_g1: G1Affine::identity(),
		beta_g2: beta2_affine.unwrap(),
		gamma_g2: gamma2_affine.unwrap(),
		delta_g1: G1Affine::identity(),
		delta_g2: delta2_affine.unwrap(),
		ic,
	})
}