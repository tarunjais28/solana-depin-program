# DePIN Program

The DePIN program is a Solana smart contract designed to manage token operations including initialization, minting, burning, staking, and unstaking of tokens. This program uses the Anchor framework and the Anchor SPL (Solana Program Library) for token management.

## Program Overview

This program exposes several functions that interact with the Solana blockchain to perform various token-related operations. The core functions provided by the program are:

- **Initialize**: Sets up the program's initial state.
- **Mint**: Creates new tokens in specified amounts.
- **Burn**: Removes tokens from circulation.
- **Stake**: Deposits tokens into a staking pool.
- **Unstake**: Withdraws tokens from the staking pool.

## Imports

- `constants::*`: Import constants used in the program.
- `errors::*`: Import error definitions for the program.
- `events::*`: Import event definitions for the program.
- `instructions::*`: Import instruction implementations.
- `states::*`: Import state management utilities.

### External Libraries

- **Anchor Lang**: Provides core functionality for building Solana smart contracts.
  - `account_info::AccountInfo`: For handling Solana account information.
  - `sysvar::Sysvar`: For accessing system variables.

- **Anchor SPL**: Provides standard token operations.
  - `associated_token::AssociatedToken`: For managing associated token accounts.
  - `token::{Burn, MintTo, Token, Transfer}`: For token operations including minting, burning, and transferring.
  - `token_interface::TokenAccount`: For interacting with token accounts.

## Program ID

The program ID for this contract is: `3W7pnY6U3Aa7ERYf7KTwMmfNmyFRNTNivR4Ya6nKScXh`. This ID uniquely identifies the deployed smart contract on the Solana blockchain.

## Functions

### `initialize`

Initializes the program's state.

```rust
pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    instructions::init(ctx)
}
```

### `mint`

Mints new DePin tokens in specified amounts in exchange of token_a, token_b, and token_c DePin tokens.

```rust
pub fn mint(ctx: Context<MintTokens>, token_a: u64, token_b: u64, token_c: u64) -> Result<()> {
    instructions::mint_tokens(ctx, token_a, token_b, token_c)
}
```

### `burn`

Burns tokens specified by the amount. Caller will get the equivalent DePin tokens such as tokenA, tokenB and tokenC based on the weightage percentage calculations

```rust
pub fn burn(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
    instructions::burn_tokens(ctx, amount)
}
```

### `stake`

Stakes DePin tokens of a specified amount.

```rust
pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
    instructions::stake_amount(ctx, amount)
}
```

### `unstake`

Unstakes tokens. This function unstakes all DePin tokens for the user with rewards and penality whichever applied.

```rust
pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
    instructions::unstake_amount(ctx)
}
```

## Setup and Dependencies

Ensure you have the following dependencies installed:
- Anchor Documentation: project-serum.github.io/anchor/
- Solana Documentation: docs.solana.com/
- Solana CLI Documentation: docs.solana.com/cli
