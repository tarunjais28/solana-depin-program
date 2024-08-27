use super::*;

/// Function to initialize the program
pub fn init(_: Context<Initialize>) -> Result<()> {
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

    #[account(mut)]
    pub payer: Signer<'info>, // The account that will pay for the creation of new accounts

    pub system_program: Program<'info, System>, // System program for creating accounts

    pub token_program: Program<'info, Token>, // Token program for handling SPL tokens
}
