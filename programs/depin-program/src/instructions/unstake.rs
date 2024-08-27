use super::*;

/// Function to unstake the tokens
///
/// This function can throw the following errors:
///   - any of the seeds not match with the actual seeds
///   - mint keys are different
///   - caller is not the owner of the tokens
pub fn unstake_amount(ctx: Context<Unstake>) -> Result<()> {
    // Get mutable references to the stake state and global state accounts.
    let stake_state = &mut ctx.accounts.stake_state;
    let global_state = &mut ctx.accounts.global_state;

    // Calculate the withdrawal amount based on the stake state's logic.
    let withdrawal_amount = stake_state.withdraw(global_state);

    // Define the seed and signer for the escrow account, required for authorized actions.
    let seeds = &[ESCROW_TAG, &[ctx.bumps.escrow_account]];
    let signer = [&seeds[..]];

    // Prepare for the token transfer by defining the necessary accounts and authority.
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_accounts = Transfer {
        from: ctx.accounts.escrow_account.to_account_info(), // Source account (escrow account holding staked tokens)
        to: ctx.accounts.user_vault.to_account_info(), // Destination account (user's vault to receive unstaked tokens)
        authority: ctx.accounts.escrow_account.to_account_info(), // Authority for the escrow account
    };

    // Execute the token transfer from the escrow account back to the user's vault.
    token::transfer(
        CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer),
        withdrawal_amount,
    )?;

    // Emit an event to signal that unstaking has occurred.
    emit!(UnstakeEvent {
        user: ctx.accounts.vault_authority.key(), // The user who unstaked
        amount: withdrawal_amount                 // The amount that was unstaked
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Unstake<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_TAG], // Seed used to derive the PDA of the global state
        bump, // Bump seed to find a valid PDA
    )]
    pub global_state: Box<Account<'info, GlobalState>>, // Global state account containing program-wide information

    #[account(
        mut,
        seeds = [LOCK_TAG, vault_authority.key().as_ref()], // Seed used to derive the PDA of the stake state
        bump, // Bump seed to find a valid PDA
    )]
    pub stake_state: Box<Account<'info, StakeState>>, // Account storing the user's stake information

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
    pub user_vault: Box<InterfaceAccount<'info, TokenAccount>>, // User's vault to receive unstaked tokens

    #[account(mut)]
    pub vault_authority: Signer<'info>, // The authority of the vault, typically the user

    pub system_program: Program<'info, System>, // Solana System Program

    pub token_program: Program<'info, Token>, // Solana Token Program for handling SPL tokens
}
