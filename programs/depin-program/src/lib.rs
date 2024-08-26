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

mod constants;
mod errors;
mod events;
mod instructions;
mod states;

declare_id!("3W7pnY6U3Aa7ERYf7KTwMmfNmyFRNTNivR4Ya6nKScXh");

#[program]
pub mod depin_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::init(ctx)
    }

    pub fn mint(ctx: Context<MintTokens>, token_a: u64, token_b: u64, token_c: u64) -> Result<()> {
        instructions::mint_tokens(ctx, token_a, token_b, token_c)
    }

    pub fn burn(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        instructions::burn_tokens(ctx, amount)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake_amount(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        instructions::unstake_amount(ctx)
    }
}
