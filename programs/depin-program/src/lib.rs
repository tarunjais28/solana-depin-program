// Import necessary modules and crates for the program.
use crate::{constants::*, errors::*, events::*, instructions::*, states::*};
use anchor_lang::{
    prelude::*,
    solana_program::{account_info::AccountInfo, sysvar::Sysvar},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Burn, MintTo, Token, Transfer},
    token_interface::TokenAccount,
};

// Declare the custom modules used within the program for constants, errors,
// events, instructions, and states.
mod constants;
mod errors;
mod events;
mod instructions;
mod states;

// Define the program ID for the Solana smart contract. This ID uniquely
// identifies the program on the Solana blockchain.
declare_id!("3W7pnY6U3Aa7ERYf7KTwMmfNmyFRNTNivR4Ya6nKScXh");

// Define the program module and its associated functions.
#[program]
pub mod depin_program {
    use super::*;

    // Function to initialize the program. Calls the corresponding
    // initialization instruction.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Delegate to the `init` function in the `instructions` module.
        instructions::init(ctx)
    }

    // Function to mint tokens. Specifies amounts for three different tokens
    // and delegates to the corresponding instruction.
    pub fn mint(ctx: Context<MintTokens>, token_a: u64, token_b: u64, token_c: u64) -> Result<()> {
        // Delegate to the `mint_tokens` function in the `instructions` module.
        instructions::mint_tokens(ctx, token_a, token_b, token_c)
    }

    // Function to burn tokens. Specifies the amount to be burned and delegates
    // to the corresponding instruction.
    pub fn burn(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        // Delegate to the `burn_tokens` function in the `instructions` module.
        instructions::burn_tokens(ctx, amount)
    }

    // Function to stake tokens. Specifies the amount to be staked and
    // delegates to the corresponding instruction.
    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        // Delegate to the `stake_amount` function in the `instructions` module.
        instructions::stake_amount(ctx, amount)
    }

    // Function to unstake tokens. No amount specified, so presumably unstakes
    // all tokens for the user and delegates to the corresponding instruction.
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        // Delegate to the `unstake_amount` function in the `instructions` module.
        instructions::unstake_amount(ctx)
    }
}
