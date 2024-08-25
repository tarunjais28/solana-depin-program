use crate::{constants::*, errors::*, events::*, instructions::*, states::*, structs::*};
use anchor_lang::{
    prelude::*,
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, rent::Rent,
        sysvar::Sysvar,
    },
    Lamports,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, set_authority, Burn, MintTo, SetAuthority, Token, TransferChecked},
    token_interface::{token_metadata_initialize, Mint, TokenAccount, TokenMetadataInitialize},
};

mod constants;
mod errors;
mod events;
mod instructions;
mod states;
mod structs;

declare_id!("3W7pnY6U3Aa7ERYf7KTwMmfNmyFRNTNivR4Ya6nKScXh");

#[program]
pub mod depin_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn create_token(ctx: Context<CreateToken>, params: CreateParams) -> Result<()> {
        instructions::create(ctx, params)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
