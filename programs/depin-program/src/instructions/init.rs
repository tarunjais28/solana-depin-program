use super::*;

/// Function to initialize program
pub fn init(ctx: Context<Initialize>) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    global_state.token_a = ctx.accounts.token_a.key();
    global_state.token_b = ctx.accounts.token_b.key();
    global_state.token_c = ctx.accounts.token_c.key();
    global_state.mint_account = ctx.accounts.mint_account.key();

    // Emit initialize event
    emit!(InitializeEvent {});

    Ok(())
}

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [GLOBAL_TAG],
        bump,
        payer = payer,
        space = std::mem::size_of::<GlobalState>() + 8
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
        init,
        token::mint = token_a,
        token::authority = escrow_account_a,
        seeds = [ESCROW_TAG, b"TokenA"],
        bump,
        payer = payer,
    )]
    pub escrow_account_a: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        token::mint = token_b,
        token::authority = escrow_account_b,
        seeds = [ESCROW_TAG, b"TokenB"],
        bump,
        payer = payer,
    )]
    pub escrow_account_b: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        token::mint = token_c,
        token::authority = escrow_account_c,
        seeds = [ESCROW_TAG, b"TokenC"],
        bump,
        payer = payer,
    )]
    pub escrow_account_c: Box<InterfaceAccount<'info, TokenAccount>>,

    /// CHECK: This is the DePIN TokenA
    pub token_a: AccountInfo<'info>,

    /// CHECK: This is the DePIN TokenB
    pub token_b: AccountInfo<'info>,

    /// CHECK: This is the DePIN TokenC
    pub token_c: AccountInfo<'info>,

    /// CHECK: This is the DePIN Token
    pub mint_account: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}
