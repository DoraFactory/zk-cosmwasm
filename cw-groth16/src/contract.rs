use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use crate::ContractError;

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
            vk_alpha1,
            vk_beta_2,
            vk_gamma_2,
            vk_delta_2,
            vk_ic0,
            vk_ic1
        } 
            => execute_set_zkeys(deps, env, info, public_signal, vk_alpha1, vk_beta_2, vk_gamma_2, vk_delta_2, vk_ic0, vk_ic1),
        ExecuteMsg::Proof { 
            proof_a,
            proof_b,
            proof_c,
        } 
            => execute_publish_proof(deps, env, info, proof_a, proof_b, proof_c),
    }
}


pub fn execute_set_zkeys(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    public_signal: Vec<u8>,
    vk_alpha1: Vec<u8>,
    vk_beta_2: Vec<u8>,
    vk_gamma_2: Vec<u8>,
    vk_delta_2: Vec<u8>,
    vk_ic0: Vec<u8>,
    vk_ic1: Vec<u8>,
) -> Result<Response, ContractError> {
    //TODO:
}

pub fn execute_publish_proof(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    proof_a: Vec<u8>,
    proof_b: Vec<u8>,
    proof_c: Vec<u8>,
) -> Result<Response, ContractError> {
    //TODO:
}

