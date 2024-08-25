use super::*;

/// Function to create token
pub fn create(ctx: Context<CreateToken>, params: CreateParams) -> Result<()> {
    ctx.accounts.initialize_token_metadata(params.clone())?;
    ctx.accounts.update_mint_authority()?;
    ctx.accounts.mint_account.reload()?;

    // transfer minimum rent to mint account
    update_account_lamports_to_minimum_balance(
        ctx.accounts.mint_account.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
    )?;

    // Emit create token event
    emit!(CreateTokenEvent { name: params.name });

    Ok(())
}

#[derive(Accounts)]
#[instruction(params: CreateParams)]
pub struct CreateToken<'info> {
    #[account(
        init,
        seeds = [MINT_TAG, params.name.as_bytes()],
        bump,
        payer = payer,
        mint::token_program = token_program,
        mint::decimals = 6,
        mint::authority = payer,
        mint::freeze_authority = payer,
        extensions::metadata_pointer::authority = payer,
        extensions::metadata_pointer::metadata_address = mint_account,
        extensions::close_authority::authority = payer,
        extensions::permanent_delegate::delegate = payer,
    )]
    pub mint_account: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}

impl<'info> CreateToken<'info> {
    #[inline(never)]
    fn initialize_token_metadata(&self, params: CreateParams) -> ProgramResult {
        let cpi_accounts = TokenMetadataInitialize {
            token_program_id: self.token_program.to_account_info(),
            mint: self.mint_account.to_account_info(),
            metadata: self.mint_account.to_account_info(), // metadata account is the mint, since data is stored in mint
            mint_authority: self.payer.to_account_info(),
            update_authority: self.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        token_metadata_initialize(cpi_ctx, params.name, params.symbol, params.uri)?;

        Ok(())
    }

    #[inline(never)]
    fn update_mint_authority(&self) -> ProgramResult {
        let cpi_accounts = SetAuthority {
            current_authority: self.payer.to_account_info(),
            account_or_mint: self.mint_account.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        set_authority(
            cpi_ctx,
            spl_token::instruction::AuthorityType::MintTokens,
            Some(self.mint_account.key()),
        )?;

        Ok(())
    }
}

#[inline(never)]
pub fn update_account_lamports_to_minimum_balance<'info>(
    account: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>,
) -> Result<()> {
    let extra_lamports = Rent::get()?.minimum_balance(account.data_len()) - account.get_lamports();
    if extra_lamports > 0 {
        invoke(
            &anchor_lang::solana_program::system_instruction::transfer(
                payer.key,
                account.key,
                extra_lamports,
            ),
            &[payer, account, system_program],
        )?;
    }
    Ok(())
}
