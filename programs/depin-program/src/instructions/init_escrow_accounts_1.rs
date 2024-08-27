use super::*;

/// Function to initialize escrow accounts
pub fn init_escrow_accounts_1(ctx: Context<InitializeEscrows1>) -> Result<()> {
    // Access and modify the global state account
    let global_state = &mut ctx.accounts.global_state;

    // Store the keys of the DePIN token TokenA and mint account
    global_state.mint_account = ctx.accounts.mint_account.key();
    global_state.token_a = ctx.accounts.token_a.key();

    // Emit an event indicating that the initialization has been completed
    emit!(InitializeEvent {});

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeEscrows1<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_TAG], // Seed used for deriving the PDA of the global state
        bump, // Bump seed to find a valid PDA
    )]
    pub global_state: Box<Account<'info, GlobalState>>, // Global state account containing program-wide information

    #[account(
        init,
        token::mint = mint_account, // Specify the mint associated with the token account
        token::authority = escrow_account, // Authority that can manage this token account
        seeds = [ESCROW_TAG], // Seeds used for deriving the PDA
        bump, // Bump seed to find a valid PDA
        payer = payer, // Specifies who will pay for the account's rent
    )]
    pub escrow_account: Box<InterfaceAccount<'info, TokenAccount>>, // Escrow account to hold tokens

    #[account(
        init,
        token::mint = token_a, // Specify the mint associated with token A
        token::authority = escrow_account_a, // Authority that can manage token A account
        seeds = [ESCROW_TAG, b"TokenA"], // Seeds used for deriving the PDA for TokenA
        bump, // Bump seed to find a valid PDA
        payer = payer, // Specifies who will pay for the account's rent
    )]
    pub escrow_account_a: Box<InterfaceAccount<'info, TokenAccount>>, // Escrow account for TokenA

    /// CHECK: This is the DePIN Mint Account
    pub mint_account: AccountInfo<'info>, // AccountInfo for the mint account, only the key is used

    /// CHECK: This is the DePIN TokenA
    pub token_a: AccountInfo<'info>, // AccountInfo for TokenA, only the key is used

    #[account(mut)]
    pub payer: Signer<'info>, // The account that will pay for the creation of new accounts

    pub system_program: Program<'info, System>, // System program for creating accounts

    pub token_program: Program<'info, Token>, // Token program for handling SPL tokens
}
