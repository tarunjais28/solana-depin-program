use super::*;

#[error_code]
pub enum CustomError {
    #[msg("Error: Amount can't be zero!")]
    AmountCantBeZero,

    #[msg("Error: Unauthorized User!")]
    Unauthorized,

    #[msg("Error: Unknown Token A Passed!")]
    UnknownTokenA,

    #[msg("Error: Unknown Token B Passed!")]
    UnknownTokenB,

    #[msg("Error: Unknown Token C Passed!")]
    UnknownTokenC,
}
