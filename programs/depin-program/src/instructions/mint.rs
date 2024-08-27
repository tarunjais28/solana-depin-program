use super::*;

/// Function to mint the tokens
///
/// This function can throw the following errors:
///   - any of the seeds not match with the actual seeds
///   - mint keys are different
///   - caller is not the owner of the tokens
pub fn mint_tokens(
    ctx: Context<MintTokens>,
    token_a: u64,
    token_b: u64,
    token_c: u64,
) -> Result<()> {
    // Calculate the required amount of DPIT token based on the provided
    // quantities of token_a, token_b, and token_c, applying their respective weightages.
    let dpit_amount = ((token_a * TOKEN_A_WEIGHTAGE)
        + (token_b * TOKEN_B_WEIGHTAGE)
        + (token_c * TOKEN_C_WEIGHTAGE))
        / 100;

    // Create the Transfer struct for token A to move it from the user's ATA
    // to the program's escrow account.
    let mut cpi_accounts = Transfer {
        from: ctx.accounts.token_a_ata.to_account_info(),
        to: ctx.accounts.escrow_account_a.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    // Perform the transfer for token A using the Solana Token Program
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        token_a,
    )?;

    // Update the Transfer struct for token B and move it to the escrow account.
    cpi_accounts = Transfer {
        from: ctx.accounts.token_b_ata.to_account_info(),
        to: ctx.accounts.escrow_account_b.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    // Perform the transfer for token B using the Solana Token Program
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        token_b,
    )?;

    // Update the Transfer struct for token C and move it to the escrow account.
    cpi_accounts = Transfer {
        from: ctx.accounts.token_c_ata.to_account_info(),
        to: ctx.accounts.escrow_account_c.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
    };

    // Perform the transfer for token C using the Solana Token Program
    token::transfer(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        token_c,
    )?;

    // Create the MintTo struct to mint the calculated amount of DPIT tokens
    // to the user's specified account.
    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint_account.to_account_info(),
        to: ctx.accounts.to_account.to_account_info(),
        authority: ctx.accounts.mint_authority.to_account_info(),
    };

    // Mint the calculated amount of DPIT tokens using the Solana Token Program
    token::mint_to(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts),
        dpit_amount,
    )?;

    // Emit an event signaling that the mint operation has completed successfully
    emit!(MintEvent {
        amount: dpit_amount
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct MintTokens<'info> {
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
    pub mint_account: AccountInfo<'info>, // Mint account used in the mint operation

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
        init_if_needed,
        payer = authority,
        associated_token::mint = mint_account,
        associated_token::authority = authority,
        associated_token::token_program = token_program,
        constraint = to_account.mint == global_state.mint_account // Ensure the destination mint matches the global state
    )]
    pub to_account: InterfaceAccount<'info, TokenAccount>, // Destination account to which tokens will be minted

    /// CHECK: The authority of the token accounts involved in the mint operation
    #[account(mut)]
    pub authority: Signer<'info>, // The authority that can manage the token accounts

    /// CHECK: The authority of the mint account involved in the mint operation
    #[account(mut)]
    pub mint_authority: Signer<'info>, // The authority that can mint tokens from the mint account

    pub token_program: Program<'info, Token>, // Solana Token Program for handling SPL tokens

    pub system_program: Program<'info, System>, // Solana System Program

    pub associated_token_program: Program<'info, AssociatedToken>, // Solana Associated Token Program for creating ATAs
}
