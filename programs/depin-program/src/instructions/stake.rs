use super::*;

/// Function to stake the tokens
///
/// This function can throw following errors:
///   - Insufficient Funds (when User vault has lesser value than staked value
///     passed by the user).
pub fn stake_amount(ctx: Context<Stake>, amount: u64) -> Result<()> {
    // Check user balance first
    require!(
        ctx.accounts.user_vault.amount >= amount,
        CustomError::InsufficientFunds
    );

    // Check staked amount to be greater than 0
    require!(amount > 0, CustomError::AmountCantBeZero);

    // Transfer tokens from user to vault's account
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_vault.to_account_info(),
        to: ctx.accounts.escrow_account.to_account_info(),
        authority: ctx.accounts.vault_authority.to_account_info(),
    };
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

    // Updating total stakers
    let global_state = &mut ctx.accounts.global_state;
    global_state.total_stakers += 1;

    // Store stake information
    let stake_state = &mut ctx.accounts.stake_state;
    stake_state.stake(amount)?;

    // Emit Stake Event
    emit!(StakeEvent {
        user: ctx.accounts.vault_authority.key(),
        amount
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Stake<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_TAG],
        bump,
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
        init_if_needed,
        seeds = [LOCK_TAG, vault_authority.key().as_ref()],
        bump,
        payer = vault_authority,
        space = std::mem::size_of::<Stake>() + 8
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
