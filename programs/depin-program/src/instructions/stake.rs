use super::*;

/// Function to stake the tokens
///
/// This function can throw the following errors:
///   - Insufficient Funds (when User vault has less value than the staked amount
///     passed by the user).
///   - staked amount is 0
///   - any of the seeds not match with the actual seeds
///   - mint keys are different
///   - caller is not the owner of the tokens
pub fn stake_amount(ctx: Context<Stake>, amount: u64) -> Result<()> {
    // Check if the user's vault has enough tokens to cover the staked amount.
    require!(
        ctx.accounts.user_vault.amount >= amount,
        CustomError::InsufficientFunds
    );

    // Ensure the staked amount is greater than 0 to proceed.
    require!(amount > 0, CustomError::AmountCantBeZero);

    // Prepare for the token transfer by defining the necessary accounts and authority.
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = Transfer {
        from: ctx.accounts.user_vault.to_account_info(), // Source account (user's vault)
        to: ctx.accounts.escrow_account.to_account_info(), // Destination account (program's escrow)
        authority: ctx.accounts.vault_authority.to_account_info(), // Authority to authorize the transfer
    };

    // Execute the token transfer from the user's vault to the escrow account.
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

    // Update the total number of stakers in the global state.
    let global_state = &mut ctx.accounts.global_state;
    global_state.total_stakers += 1;

    // Record the stake information in the stake state account.
    let stake_state = &mut ctx.accounts.stake_state;
    stake_state.stake(amount)?;

    // Emit an event to signal that staking has occurred.
    emit!(StakeEvent {
        user: ctx.accounts.vault_authority.key(), // The user who staked
        amount                                    // The amount that was staked
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Stake<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_TAG], // Seed used to derive the PDA of the global state
        bump, // Bump seed to find a valid PDA
    )]
    pub global_state: Box<Account<'info, GlobalState>>, // Global state account containing program-wide information

    #[account(
        init_if_needed, // Initialize the stake state account if it doesn't exist
        seeds = [LOCK_TAG, vault_authority.key().as_ref()], // Seed used to derive the PDA of the stake state
        bump, // Bump seed to find a valid PDA
        payer = vault_authority, // The vault authority pays for the account creation
        space = std::mem::size_of::<Stake>() + 8 // Allocate sufficient space for the stake state account
    )]
    pub stake_state: Box<Account<'info, StakeState>>, // Account to store the user's stake information

    #[account(
        mut,
        seeds = [ESCROW_TAG], // Seed used to derive the PDA of the escrow account
        bump, // Bump seed to find a valid PDA
    )]
    pub escrow_account: Box<InterfaceAccount<'info, TokenAccount>>, // Escrow account holding the staked tokens

    #[account(
        mut,
        constraint = vault_authority.key() == user_vault.owner, // Ensure the vault authority owns the user vault
        constraint = user_vault.mint == global_state.mint_account // Ensure the vault is for the correct token mint
    )]
    pub user_vault: Box<InterfaceAccount<'info, TokenAccount>>, // User's vault from which tokens will be staked

    #[account(mut)]
    pub vault_authority: Signer<'info>, // The authority of the vault, typically the user

    pub system_program: Program<'info, System>, // Solana System Program

    pub token_program: Program<'info, Token>, // Solana Token Program for handling SPL tokens
}
