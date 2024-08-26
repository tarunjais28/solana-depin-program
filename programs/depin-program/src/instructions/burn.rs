use super::*;

/// Function to burn the tokens
///
/// This function can throw following errors:
pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
    // Calculate the required amount of DPIT tokens
    let token_a = amount * TOKEN_A_WEIGHTAGE / 100;
    let token_b = amount * TOKEN_B_WEIGHTAGE / 100;
    let token_c = amount * TOKEN_C_WEIGHTAGE / 100;

    let seeds = &[ESCROW_TAG, b"TokenA", &[ctx.bumps.escrow_account_a]];
    let signer = [&seeds[..]];

    // Create the Transfer struct for our context
    let mut cpi_accounts: Transfer = Transfer {
        from: ctx.accounts.escrow_account_a.to_account_info(),
        to: ctx.accounts.token_a_ata.to_account_info(),
        authority: ctx.accounts.escrow_account_a.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &signer,
        ),
        token_a,
    )?;

    let seeds = &[ESCROW_TAG, b"TokenB", &[ctx.bumps.escrow_account_b]];
    let signer = [&seeds[..]];

    cpi_accounts = Transfer {
        from: ctx.accounts.escrow_account_b.to_account_info(),
        to: ctx.accounts.token_b_ata.to_account_info(),
        authority: ctx.accounts.escrow_account_b.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &signer,
        ),
        token_b,
    )?;

    let seeds = &[ESCROW_TAG, b"TokenC", &[ctx.bumps.escrow_account_c]];
    let signer = [&seeds[..]];

    cpi_accounts = Transfer {
        from: ctx.accounts.escrow_account_c.to_account_info(),
        to: ctx.accounts.token_c_ata.to_account_info(),
        authority: ctx.accounts.escrow_account_c.to_account_info(),
    };

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &signer,
        ),
        token_c,
    )?;

    // Create the Burn struct for our context
    let cpi_accounts = Burn {
        mint: ctx.accounts.mint_account.to_account_info(),
        from: ctx.accounts.from_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    token::burn(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        amount,
    )?;

    // Emit burn event
    emit!(BurnEvent {
        token_a,
        token_b,
        token_c
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct BurnTokens<'info> {
    #[account(
        seeds = [GLOBAL_TAG],
        bump,
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    /// CHECK: This is the token that we want to mint
    #[account(
        mut,
        constraint = *mint_account.key == global_state.mint_account
    )]
    pub mint_account: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [ESCROW_TAG, b"TokenA"],
        bump,
    )]
    pub escrow_account_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [ESCROW_TAG, b"TokenB"],
        bump,
    )]
    pub escrow_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [ESCROW_TAG, b"TokenC"],
        bump,
    )]
    pub escrow_account_c: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: This is the token that we get from user
    #[account(
        mut,
        constraint = token_a_ata.mint == global_state.token_a
    )]
    pub token_a_ata: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: This is the token that we get from user
    #[account(
        mut,
        constraint = token_b_ata.mint == global_state.token_b
    )]
    pub token_b_ata: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: This is the token that we get from user
    #[account(
        mut,
        constraint = token_c_ata.mint == global_state.token_c
    )]
    pub token_c_ata: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: This is the token account that we want to mint tokens to (ATA)
    #[account(
        mut,
        constraint = from_account.mint == global_state.mint_account
    )]
    pub from_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: the authority of the token accounts
    #[account(mut)]
    pub authority: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}
