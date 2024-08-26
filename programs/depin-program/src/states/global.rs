use super::*;

/// The struct for storing global configuration
#[account]
pub struct GlobalState {
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub token_c: Pubkey,
    pub mint_account: Pubkey,
    pub total_stakers: u64,
}
