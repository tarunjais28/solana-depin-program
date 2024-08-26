use super::*;

/// Function to unstake the tokens
///
/// This function can throw following errors:
///   - Insufficient Funds (when withdrawal amount has greater value than
///     staked value).
pub fn unstake_amount(ctx: Context<Unstake>) -> Result<()> {
    let stake_state = &mut ctx.accounts.stake_state;
    let global_state = &mut ctx.accounts.global_state;

    let withdrawal_amount = stake_state.withdraw(global_state);

    let seeds = &[ESCROW_TAG, &[ctx.bumps.escrow_account]];
    let signer = [&seeds[..]];

    // Transfer staked amount from vault to user's account
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = Transfer {
        from: ctx.accounts.escrow_account.to_account_info(),
        to: ctx.accounts.user_vault.to_account_info(),
        authority: ctx.accounts.escrow_account.to_account_info(),
    };
    token::transfer(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer),
        withdrawal_amount,
    )?;

    // Emit Unstake Event
    emit!(UnstakeEvent {
        user: ctx.accounts.vault_authority.key(),
        amount: withdrawal_amount
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Unstake<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_TAG],
        bump,
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
        mut,
        seeds = [LOCK_TAG, vault_authority.key().as_ref()],
        bump,
    )]
    pub stake_state: Box<Account<'info, StakeState>>,

    #[account(
        mut,
        seeds = [ESCROW_TAG],
        bump,
    )]
    pub escrow_account: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = vault_authority.key() == user_vault.owner,
        constraint = user_vault.mint == global_state.mint_account
    )]
    pub user_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub vault_authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}
