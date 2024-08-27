use super::*;

/// Constant tag used to identify the global state account
#[constant]
pub const GLOBAL_TAG: &[u8] = b"global";

/// Constant tag used to identify escrow accounts
#[constant]
pub const ESCROW_TAG: &[u8] = b"escrow";

/// Constant tag used to identify lock accounts related to staking
#[constant]
pub const LOCK_TAG: &[u8] = b"lock";

/// Weightage (percentage) assigned to Token A in the calculations
#[constant]
pub const TOKEN_A_WEIGHTAGE: u64 = 40;

/// Weightage (percentage) assigned to Token B in the calculations
#[constant]
pub const TOKEN_B_WEIGHTAGE: u64 = 30;

/// Weightage (percentage) assigned to Token C in the calculations
#[constant]
pub const TOKEN_C_WEIGHTAGE: u64 = 30;

/// Constant representing the number of seconds in a day (used for time-based calculations)
#[constant]
pub const SECONDS_PER_DAY: u64 = 60 * 60 * 24;
