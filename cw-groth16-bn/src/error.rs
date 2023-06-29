use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Insufficient funds sent")]
    InsufficientFundsSend {},

    #[error("error verification")]
    ErrorVerificationKey {},

    #[error("error proof")]
    ErrorProof {},

    #[error("error public signal")]
    ErrorPublicSignal {},

    #[error("no verification key")]
    NoVerificationKey {},

    #[error("no public signal")]
    NoPublicSignal {},

    #[error("parse public signal error")]
    ParsePulbicSignalError {},

    #[error("invalid proof, verify failed")]
    InvalidProof {},

    #[error("this account({difficuty_issuer}) didn't issue difficulty problem")]
    NonPublishDifficulty { difficuty_issuer: String },

    #[error("invalid hex format")]
    HexDecodingError {},
}
