use super::*;

/// Function to initialize escrow accounts
pub fn init_escrow_accounts_2(ctx: Context<InitializeEscrows2>) -> Result<()> {
    // Access and modify the global state account
    let global_state = &mut ctx.accounts.global_state;

    // Store the keys of the DePIN tokens (TokenB, TokenC)
    global_state.token_b = ctx.accounts.token_b.key();
    global_state.token_c = ctx.accounts.token_c.key();

    // Emit an event indicating that the initialization has been completed
    emit!(InitializeEvent {});

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeEscrows2<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_TAG], // Seed used for deriving the PDA of the global state
        bump, // Bump seed to find a valid PDA
    )]
    pub global_state: Box<Account<'info, GlobalState>>, // Global state account containing program-wide information

    #[account(
        init,
        token::mint = token_b, // Specify the mint associated with token B
        token::authority = escrow_account_b, // Authority that can manage token B account
        seeds = [ESCROW_TAG, b"TokenB"], // Seeds used for deriving the PDA for TokenB
        bump, // Bump seed to find a valid PDA
        payer = payer, // Specifies who will pay for the account's rent
    )]
    pub escrow_account_b: Box<InterfaceAccount<'info, TokenAccount>>, // Escrow account for TokenB

    #[account(
        init,
        token::mint = token_c, // Specify the mint associated with token C
        token::authority = escrow_account_c, // Authority that can manage token C account
        seeds = [ESCROW_TAG, b"TokenC"], // Seeds used for deriving the PDA for TokenC
        bump, // Bump seed to find a valid PDA
        payer = payer, // Specifies who will pay for the account's rent
    )]
    pub escrow_account_c: Box<InterfaceAccount<'info, TokenAccount>>, // Escrow account for TokenC

    /// CHECK: This is the DePIN TokenB
    pub token_b: AccountInfo<'info>, // AccountInfo for TokenB, only the key is used

    /// CHECK: This is the DePIN TokenC
    pub token_c: AccountInfo<'info>, // AccountInfo for TokenC, only the key is used

    #[account(mut)]
    pub payer: Signer<'info>, // The account that will pay for the creation of new accounts

    pub system_program: Program<'info, System>, // System program for creating accounts

    pub token_program: Program<'info, Token>, // Token program for handling SPL tokens
}
