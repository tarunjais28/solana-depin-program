// Importing necessary items from the parent module or crate.
use super::*;

// Define a custom error enumeration to handle specific error cases.
#[error_code]
pub enum CustomError {
    // Error variant indicating that the provided amount is zero, which is not allowed.
    #[msg("Error: Amount can't be zero!")]
    AmountCantBeZero,

    // Error variant for unauthorized access or operations by a user.
    #[msg("Error: Unauthorized User!")]
    Unauthorized,

    // Error variant when a user has insufficient funds for a transaction or operation.
    #[msg("Error: Your balance is not enough.")]
    InsufficientFunds,
}
