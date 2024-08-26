use super::*;

/// Function to mint the tokens
///
/// This function can throw following errors:
pub fn mint_tokens(
    ctx: Context<MintTokens>,
    token_a: u64,
    token_b: u64,
    token_c: u64,
) -> Result<()> {
    // Calculate the required amount of DPIT token
    let dpit_amount = ((token_a * TOKEN_A_WEIGHTAGE)
        + (token_b * TOKEN_B_WEIGHTAGE)
        + (token_c * TOKEN_C_WEIGHTAGE))
        / 100;

    // Create the Transfer struct for our context
    let mut cpi_accounts = Transfer {
        from: ctx.accounts.token_a_ata.to_account_info(),
        to: ctx.accounts.escrow_account_a.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        token_a,
    )?;

    cpi_accounts = Transfer {
        from: ctx.accounts.token_b_ata.to_account_info(),
        to: ctx.accounts.escrow_account_b.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        token_b,
    )?;

    cpi_accounts = Transfer {
        from: ctx.accounts.token_c_ata.to_account_info(),
        to: ctx.accounts.escrow_account_c.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        token_c,
    )?;

    // Create the MintTo struct for our context
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint_account.to_account_info(),
        to: ctx.accounts.to_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };

    token::mint_to(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        dpit_amount,
    )?;

    // Emit mint event
    emit!(MintEvent {
        amount: dpit_amount
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct MintTokens<'info> {
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
        constraint = to_account.mint == global_state.mint_account
    )]
    pub to_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: the authority of the token accounts
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: the authority of the mint account
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}
