use super::*;

/// Function to initialize the program
pub fn init(ctx: Context<Initialize>) -> Result<()> {
    // Access and modify the global state account
    let global_state = &mut ctx.accounts.global_state;

    // Store the keys of the DePIN tokens (TokenA, TokenB, TokenC) and the mint account
    global_state.token_a = ctx.accounts.token_a.key();
    global_state.token_b = ctx.accounts.token_b.key();
    global_state.token_c = ctx.accounts.token_c.key();
    global_state.mint_account = ctx.accounts.mint_account.key();

    // Emit an event indicating that the initialization has been completed
    emit!(InitializeEvent {});

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(
        init, // Indicates that this account should be created and initialized
        seeds = [GLOBAL_TAG], // Seeds used for deriving the PDA (Program Derived Address)
        bump, // Bump seed to find a valid PDA
        payer = payer, // Specifies who will pay for the account's rent
        space = std::mem::size_of::<GlobalState>() + 8 // Allocate space for the GlobalState struct plus some extra space
    )]
    pub global_state: Box<Account<'info, GlobalState>>, // The global state account to be initialized

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

    /// CHECK: This is the DePIN TokenA
    pub token_a: AccountInfo<'info>, // AccountInfo for TokenA, only the key is used

    /// CHECK: This is the DePIN TokenB
    pub token_b: AccountInfo<'info>, // AccountInfo for TokenB, only the key is used

    /// CHECK: This is the DePIN TokenC
    pub token_c: AccountInfo<'info>, // AccountInfo for TokenC, only the key is used

    /// CHECK: This is the DePIN Mint Account
    pub mint_account: AccountInfo<'info>, // AccountInfo for the mint account, only the key is used

    #[account(mut)]
    pub payer: Signer<'info>, // The account that will pay for the creation of new accounts

    pub system_program: Program<'info, System>, // System program for creating accounts

    pub token_program: Program<'info, Token>, // Token program for handling SPL tokens
}
