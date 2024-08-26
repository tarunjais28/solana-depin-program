use super::*;

#[error_code]
pub enum CustomError {
    #[msg("Error: Amount can't be zero!")]
    AmountCantBeZero,

    #[msg("Error: Unauthorized User!")]
    Unauthorized,

    #[msg("Error: Your balance is not enough.")]
    InsufficientFunds,
}
