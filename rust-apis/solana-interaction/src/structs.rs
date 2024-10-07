use super::*;

pub struct Tokens {
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub token_c: Pubkey,
    pub dpit: Pubkey,
}

impl Tokens {
    pub fn load() -> Self {
        Self {
            token_a: Pubkey::from_str("9yj32Bk5Jv2jSG2fMw9P8TEvMNUcwbf5KZR5RWZJMTEX").unwrap(),
            token_b: Pubkey::from_str("GRTh3GajaetVDCZeTEPn7SC45nqUcYCvh4gNjvnYhQoB").unwrap(),
            token_c: Pubkey::from_str("8aoS8p9xHD3cLKgAjEngCSCcjk9wf2dXtarYjThQCTdP").unwrap(),
            dpit: Pubkey::from_str("CwmJZE2ByVbxEeB6UDcn2dQDpv2rkNUAhRy5PtoCrd5F").unwrap(),
        }
    }
}

pub struct PdaAccounts {
    pub global: Pubkey,
    pub escrow: Pubkey,
    pub escrow_a: Pubkey,
    pub escrow_b: Pubkey,
    pub escrow_c: Pubkey,
}

impl PdaAccounts {
    pub fn load(program_id: &Pubkey) -> Self {
        Self {
            global: get_pda(&["global".to_string().as_bytes()], program_id),
            escrow: get_pda(&["escrow".to_string().as_bytes()], program_id),
            escrow_a: get_pda(
                &[
                    "escrow".to_string().as_bytes(),
                    "TokenA".to_string().as_bytes(),
                ],
                program_id,
            ),
            escrow_b: get_pda(
                &[
                    "escrow".to_string().as_bytes(),
                    "TokenB".to_string().as_bytes(),
                ],
                program_id,
            ),
            escrow_c: get_pda(
                &[
                    "escrow".to_string().as_bytes(),
                    "TokenC".to_string().as_bytes(),
                ],
                program_id,
            ),
        }
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
pub struct GlobalState {
    pub token_a: Pubkey,      // Public key of Token A (likely a specific SPL token)
    pub token_b: Pubkey,      // Public key of Token B (likely a specific SPL token)
    pub token_c: Pubkey,      // Public key of Token C (likely a specific SPL token)
    pub mint_account: Pubkey, // Public key of the mint account associated with a specific SPL token
    pub total_stakers: u64,   // Total number of stakers in the program
    pub amount_after_penality: u64, // The amount after applying any penalties (e.g., early withdrawal penalties)
}

/// The struct containing instructions for staking
#[derive(AnchorDeserialize, AnchorSerialize, Debug)]
// Automatically provides a default value for the struct's fields
pub struct StakeState {
    /// Initial staked amount by the user
    pub staked_amount: u64,

    /// Timestamp of when the tokens were staked
    pub staked_at: i64,

    /// Rewards earned based on staking duration
    pub rewards: u64,

    /// Penalty applied for early withdrawal
    pub penality: u64,
}
