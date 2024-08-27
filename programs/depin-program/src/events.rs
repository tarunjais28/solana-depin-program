// Importing necessary items from the parent module or crate.
use super::*;

// Define an event for initialization purposes.
#[event]
pub struct InitializeEvent {}

// Define an event for creating a new token, including the token's name.
#[event]
pub struct CreateTokenEvent {
    /// The name of the token being created.
    pub name: String,
}

// Define an event for minting tokens, specifying the amount of tokens minted.
#[event]
pub struct MintEvent {
    // The amount of tokens minted.
    pub amount: u64,
}

// Define an event for burning tokens, specifying the amounts of different tokens being burned.
#[event]
pub struct BurnEvent {
    // Amount of Token A burned.
    pub token_a: u64,

    // Amount of Token B burned.
    pub token_b: u64,

    // Amount of Token C burned.
    pub token_c: u64,
}

// Define an event for staking tokens, including the user and the amount staked.
#[event]
pub struct StakeEvent {
    // Public key of the user staking tokens.
    pub user: Pubkey,
    // Amount of tokens being staked.
    pub amount: u64,
}

// Define an event for unstaking tokens, including the user and the amount unstaked.
#[event]
pub struct UnstakeEvent {
    // Public key of the user unstaking tokens.
    pub user: Pubkey,
    // Amount of tokens being unstaked.
    pub amount: u64,
}
