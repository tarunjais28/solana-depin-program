use super::*;

/// Function to burn the tokens
///
/// This function can throw the following errors:
///   - any of the seeds not match with the actual seeds
///   - mint keys are different
///   - caller is not the owner of the tokens
pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
    // Calculate the required amount of each type of DPIT token (A, B, C)
    // based on their respective weightage.
    let token_a = amount * TOKEN_A_WEIGHTAGE / 100;
    let token_b = amount * TOKEN_B_WEIGHTAGE / 100;
    let token_c = amount * TOKEN_C_WEIGHTAGE / 100;

    // Define the seeds and signer for token A, which will be used for the transfer.
    let seeds = &[ESCROW_TAG, b"TokenA", &[ctx.bumps.escrow_account_a]];
    let signer = [&seeds[..]];

    // Create the Transfer struct for token A
    let mut cpi_accounts: Transfer = Transfer {
        from: ctx.accounts.escrow_account_a.to_account_info(),
        to: ctx.accounts.token_a_ata.to_account_info(),
        authority: ctx.accounts.escrow_account_a.to_account_info(),
    };

    // Perform the transfer for token A using the Solana Token Program
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &signer,
        ),
        token_a,
    )?;

    // Define the seeds and signer for token B
    let seeds = &[ESCROW_TAG, b"TokenB", &[ctx.bumps.escrow_account_b]];
    let signer = [&seeds[..]];

    // Update the Transfer struct for token B
    cpi_accounts = Transfer {
        from: ctx.accounts.escrow_account_b.to_account_info(),
        to: ctx.accounts.token_b_ata.to_account_info(),
        authority: ctx.accounts.escrow_account_b.to_account_info(),
    };

    // Perform the transfer for token B using the Solana Token Program
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &signer,
        ),
        token_b,
    )?;

    // Define the seeds and signer for token C
    let seeds = &[ESCROW_TAG, b"TokenC", &[ctx.bumps.escrow_account_c]];
    let signer = [&seeds[..]];

    // Update the Transfer struct for token C
    cpi_accounts = Transfer {
        from: ctx.accounts.escrow_account_c.to_account_info(),
        to: ctx.accounts.token_c_ata.to_account_info(),
        authority: ctx.accounts.escrow_account_c.to_account_info(),
    };

    // Perform the transfer for token C using the Solana Token Program
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_accounts,
            &signer,
        ),
        token_c,
    )?;

    // Create the Burn struct for burning the specified amount of tokens
    let cpi_accounts = Burn {
        mint: ctx.accounts.mint_account.to_account_info(),
        from: ctx.accounts.from_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    // Burn the specified amount of tokens using the Solana Token Program
    token::burn(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        amount,
    )?;

    // Emit an event to signal that the burn operation has completed successfully
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
        seeds = [GLOBAL_TAG], // Seed used for deriving the PDA of the global state
        bump, // Bump seed to find a valid PDA
    )]
    pub global_state: Box<Account<'info, GlobalState>>, // Global state account containing program-wide information

    /// CHECK: This is the mint account associated with the DPIT token
    #[account(
        mut,
        constraint = *mint_account.key == global_state.mint_account // Ensure mint account matches the global state
    )]
    pub mint_account: AccountInfo<'info>, // Mint account used in the burn operation

    #[account(
        mut,
        seeds = [ESCROW_TAG, b"TokenA"], // Seed used for deriving the PDA of escrow account A
        bump, // Bump seed to find a valid PDA
    )]
    pub escrow_account_a: Box<InterfaceAccount<'info, TokenAccount>>, // Escrow account holding TokenA

    #[account(
        mut,
        seeds = [ESCROW_TAG, b"TokenB"], // Seed used for deriving the PDA of escrow account B
        bump, // Bump seed to find a valid PDA
    )]
    pub escrow_account_b: Box<InterfaceAccount<'info, TokenAccount>>, // Escrow account holding TokenB

    #[account(
        mut,
        seeds = [ESCROW_TAG, b"TokenC"], // Seed used for deriving the PDA of escrow account C
        bump, // Bump seed to find a valid PDA
    )]
    pub escrow_account_c: Box<InterfaceAccount<'info, TokenAccount>>, // Escrow account holding TokenC

    /// CHECK: This is the Token A account owned by the user
    #[account(
        mut,
        constraint = token_a_ata.mint == global_state.token_a // Ensure TokenA mint matches the global state
    )]
    pub token_a_ata: InterfaceAccount<'info, TokenAccount>, // User's TokenA associated token account (ATA)

    /// CHECK: This is the Token B account owned by the user
    #[account(
        mut,
        constraint = token_b_ata.mint == global_state.token_b // Ensure TokenB mint matches the global state
    )]
    pub token_b_ata: InterfaceAccount<'info, TokenAccount>, // User's TokenB associated token account (ATA)

    /// CHECK: This is the Token C account owned by the user
    #[account(
        mut,
        constraint = token_c_ata.mint == global_state.token_c // Ensure TokenC mint matches the global state
    )]
    pub token_c_ata: InterfaceAccount<'info, TokenAccount>, // User's TokenC associated token account (ATA)

    /// CHECK: This is the token account that we want to mint tokens to (ATA)
    #[account(
        mut,
        constraint = from_account.mint == global_state.mint_account // Ensure the source mint matches the global state
    )]
    pub from_account: InterfaceAccount<'info, TokenAccount>, // Source account from which tokens will be burned

    /// CHECK: The authority of the token accounts involved in the burn operation
    #[account(mut)]
    pub authority: Signer<'info>, // The authority that can manage the token accounts

    pub token_program: Program<'info, Token>, // Solana Token Program for handling SPL tokens

    pub system_program: Program<'info, System>, // Solana System Program
}
