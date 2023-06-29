use super::error::ContractError;
use crate::state::{ProofStr, VkeyStr};
use bellman_ce_verifier::{Proof, VerifyingKey};
use cosmwasm_std::ensure;
use pairing_ce::bn256::{G1Affine, G1Uncompressed, G2Affine, G2Uncompressed};
use pairing_ce::{CurveAffine, EncodedPoint, Engine};

/// convert the proof into the affine type, which will be used to verify
pub fn parse_proof<E>(pof: ProofStr) -> Result<Proof<E>, ContractError>
where
    E: Engine<G1Affine = G1Affine, G2Affine = G2Affine>,
{
    let pi_a = pof.pi_a;
    let pi_b = pof.pi_b;
    let pi_c = pof.pi_c;

    ensure!(pi_a.len() == 64, ContractError::ErrorProof {});
    ensure!(pi_b.len() == 128, ContractError::ErrorProof {});
    ensure!(pi_c.len() == 64, ContractError::ErrorProof {});

    let mut a_arr: [u8; 64] = [0; 64];
    let mut b_arr: [u8; 128] = [0; 128];
    let mut c_arr: [u8; 64] = [0; 64];

    a_arr[..pi_a.len()].copy_from_slice(&pi_a[..]);

    b_arr[..pi_b.len()].copy_from_slice(&pi_b[..]);

    c_arr[..pi_c.len()].copy_from_slice(&pi_c[..]);

    let pia_affine: G1Affine = G1Uncompressed::to_g1_uncompressed(a_arr)
        .into_affine()
        .map_err(|_| ContractError::ErrorProof {})?;
    let pib_affine: G2Affine = G2Uncompressed::to_g2_uncompressed(b_arr)
        .into_affine()
        .map_err(|_| ContractError::ErrorProof {})?;
    let pic_affine: G1Affine = G1Uncompressed::to_g1_uncompressed(c_arr)
        .into_affine()
        .map_err(|_| ContractError::ErrorProof {})?;

    Ok(Proof {
        a: pia_affine,
        b: pib_affine,
        c: pic_affine,
    })
}

/// convert the verification key into the affine type, which will be used in verification
pub fn parse_vkey<E>(vk: VkeyStr) -> Result<VerifyingKey<E>, ContractError>
where
    E: Engine<G1Affine = G1Affine, G2Affine = G2Affine>,
{
    let vk_alpha_1 = vk.alpha_1;
    let vk_beta_2 = vk.beta_2;
    let vk_gamma_2 = vk.gamma_2;
    let vk_delta_2 = vk.delta_2;
    let vk_ic0 = vk.ic0;
    let vk_ic1 = vk.ic1;

    ensure!(
        vk_alpha_1.len() == 64,
        ContractError::ErrorVerificationKey {}
    );
    ensure!(
        vk_beta_2.len() == 128,
        ContractError::ErrorVerificationKey {}
    );
    ensure!(
        vk_gamma_2.len() == 128,
        ContractError::ErrorVerificationKey {}
    );
    ensure!(
        vk_delta_2.len() == 128,
        ContractError::ErrorVerificationKey {}
    );
    ensure!(vk_ic0.len() == 64, ContractError::ErrorVerificationKey {});
    ensure!(vk_ic1.len() == 64, ContractError::ErrorVerificationKey {});

    let mut alpha1: [u8; 64] = [0; 64];
    let mut beta2: [u8; 128] = [0; 128];
    let mut gamma2: [u8; 128] = [0; 128];
    let mut delta2: [u8; 128] = [0; 128];
    let mut ic_0: [u8; 64] = [0; 64];
    let mut ic_1: [u8; 64] = [0; 64];
    let mut ic = Vec::new();

    alpha1[..vk_alpha_1.len()].copy_from_slice(&vk_alpha_1[..]);

    beta2[..vk_beta_2.len()].copy_from_slice(&vk_beta_2[..]);

    gamma2[..vk_gamma_2.len()].copy_from_slice(&vk_gamma_2[..]);

    delta2[..vk_delta_2.len()].copy_from_slice(&vk_delta_2[..]);

    ic_0[..vk_ic0.len()].copy_from_slice(&vk_ic0[..]);

    ic_1[..vk_ic1.len()].copy_from_slice(&vk_ic1[..]);

    let alpha1_affine = G1Uncompressed::to_g1_uncompressed(alpha1)
        .into_affine()
        .map_err(|_| ContractError::ErrorVerificationKey {})?;
    let beta2_affine = G2Uncompressed::to_g2_uncompressed(beta2)
        .into_affine()
        .map_err(|_| ContractError::ErrorVerificationKey {})?;
    let gamma2_affine = G2Uncompressed::to_g2_uncompressed(gamma2)
        .into_affine()
        .map_err(|_| ContractError::ErrorVerificationKey {})?;
    let delta2_affine = G2Uncompressed::to_g2_uncompressed(delta2)
        .into_affine()
        .map_err(|_| ContractError::ErrorVerificationKey {})?;
    let ic0_affine = G1Uncompressed::to_g1_uncompressed(ic_0)
        .into_affine()
        .map_err(|_| ContractError::ErrorVerificationKey {})?;
    let ic1_affine = G1Uncompressed::to_g1_uncompressed(ic_1)
        .into_affine()
        .map_err(|_| ContractError::ErrorVerificationKey {})?;

    ic.push(ic0_affine);
    ic.push(ic1_affine);

    // return verification key
    Ok(VerifyingKey {
        alpha_g1: alpha1_affine,
        beta_g1: G1Affine::zero(),
        beta_g2: beta2_affine,
        gamma_g2: gamma2_affine,
        delta_g1: G1Affine::zero(),
        delta_g2: delta2_affine,
        ic,
    })
}
