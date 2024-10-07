use super::*;

/// The struct for storing global configuration
#[account]
#[derive(Debug)]
pub struct GlobalState {
    pub token_a: Pubkey,      // Public key of Token A (likely a specific SPL token)
    pub token_b: Pubkey,      // Public key of Token B (likely a specific SPL token)
    pub token_c: Pubkey,      // Public key of Token C (likely a specific SPL token)
    pub mint_account: Pubkey, // Public key of the mint account associated with a specific SPL token
    pub total_stakers: u64,   // Total number of stakers in the program
    pub amount_after_penality: u64, // The amount after applying any penalties (e.g., early withdrawal penalties)
}
