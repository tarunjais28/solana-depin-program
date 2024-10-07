use spl_token::solana_program::native_token::LAMPORTS_PER_SOL;

use super::*;

async fn interact_with_solana_program(
    client: &RpcClient,
    program_id: Pubkey,
    user_keypair: &Keypair,
) {
    let recipient_pubkey = Pubkey::from_str("RECIPIENT_PUBLIC_KEY").unwrap();

    // Create an instruction to transfer SOL
    let instruction = system_instruction::transfer(
        &user_keypair.pubkey(),
        &recipient_pubkey,
        1_000_000_000, // Transfer 1 SOL (in lamports)
    );

    // Build the transaction
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&user_keypair.pubkey()));

    // Fetch the latest blockhash
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[user_keypair], recent_blockhash);

    // Send the transaction
    let signature = client
        .send_and_confirm_transaction_with_spinner_and_config(
            &transaction,
            CommitmentConfig::confirmed(),
            RpcSendTransactionConfig {
                skip_preflight: true,
                preflight_commitment: None,
                encoding: None,
                max_retries: None,
                min_context_slot: None,
            },
        )
        .expect("Transaction failed");

    println!("Transaction signature: {:?}", signature);
}

pub async fn mint_tokens(
    program: &anchor_client::Program<Rc<Keypair>>,
    pda: &PdaAccounts,
    tokens: &Tokens,
    user: &Pubkey,
) {
    let token_a_ata = get_associated_token_address(&user, &tokens.token_a);
    let token_b_ata = get_associated_token_address(&user, &tokens.token_b);
    let token_c_ata = get_associated_token_address(&user, &tokens.token_c);
    let dpit_ata = get_associated_token_address(&user, &tokens.dpit);

    let token_a = 10 * LAMPORTS_PER_SOL;
    let token_b = 10 * LAMPORTS_PER_SOL;
    let token_c = 10 * LAMPORTS_PER_SOL;

    // Invoke the method on the contract
    let tx = program
        .request()
        .accounts(depin_program::accounts::MintTokens {
            global_state: pda.global,
            mint_account: tokens.dpit,
            escrow_account_a: pda.escrow_a,
            escrow_account_b: pda.escrow_b,
            escrow_account_c: pda.escrow_c,
            token_a_ata,
            token_b_ata,
            token_c_ata,
            to_account: dpit_ata,
            authority: *user,
            mint_authority: *user,
            token_program: spl_token::id(),
            system_program: system_program::id(),
            associated_token_program: associated_token::ID,
        })
        .args(depin_program::instruction::Mint {
            token_a,
            token_b,
            token_c,
        })
        .send()
        .await
        .unwrap();

    println!("{}", format!("Tx: {}", tx).bright_white());
}

pub async fn burn_tokens(
    program: &anchor_client::Program<Rc<Keypair>>,
    pda: &PdaAccounts,
    tokens: &Tokens,
    user: &Pubkey,
) {
    let token_a_ata = get_associated_token_address(&user, &tokens.token_a);
    let token_b_ata = get_associated_token_address(&user, &tokens.token_b);
    let token_c_ata = get_associated_token_address(&user, &tokens.token_c);
    let from_account = get_associated_token_address(&user, &tokens.dpit);

    let amount = 2 * LAMPORTS_PER_SOL;

    // Invoke the method on the contract
    let tx = program
        .request()
        .accounts(depin_program::accounts::BurnTokens {
            global_state: pda.global,
            mint_account: tokens.dpit,
            escrow_account_a: pda.escrow_a,
            escrow_account_b: pda.escrow_b,
            escrow_account_c: pda.escrow_c,
            token_a_ata,
            token_b_ata,
            token_c_ata,
            from_account,
            authority: *user,
            token_program: spl_token::id(),
            system_program: system_program::id(),
        })
        .args(depin_program::instruction::Burn {
            amount,
        })
        .send()
        .await
        .unwrap();

    println!("{}", format!("Tx: {}", tx).bright_white());
}

pub async fn stake(
    program: &anchor_client::Program<Rc<Keypair>>,
    pda: &PdaAccounts,
    tokens: &Tokens,
    user: &Pubkey,
) {
    let user_ata = get_associated_token_address(&user, &tokens.dpit);
    let pda_staked_account = get_pda(
        &["lock".to_string().as_bytes(), &user.to_bytes()],
        &depin_program::id(),
    );

    let amount = 2 * LAMPORTS_PER_SOL;

    // Invoke the method on the contract
    let tx = program
        .request()
        .accounts(depin_program::accounts::Stake {
            global_state: pda.global,
            token_program: spl_token::id(),
            system_program: system_program::id(),
            stake_state: pda_staked_account,
            escrow_account: pda.escrow,
            user_vault: user_ata,
            vault_authority: *user,
        })
        .args(depin_program::instruction::Stake {
            amount,
        })
        .send()
        .await
        .unwrap();

    println!("{}", format!("Tx: {}", tx).bright_yellow());
}

pub async fn unstake(
    program: &anchor_client::Program<Rc<Keypair>>,
    pda: &PdaAccounts,
    tokens: &Tokens,
    user: &Pubkey,
) {
    let user_ata = get_associated_token_address(&user, &tokens.dpit);
    let pda_staked_account = get_pda(
        &["lock".to_string().as_bytes(), &user.to_bytes()],
        &depin_program::id(),
    );

    // Invoke the method on the contract
    let tx = program
        .request()
        .accounts(depin_program::accounts::Unstake {
            global_state: pda.global,
            token_program: spl_token::id(),
            system_program: system_program::id(),
            stake_state: pda_staked_account,
            escrow_account: pda.escrow,
            user_vault: user_ata,
            vault_authority: *user,
        })
        .send()
        .await
        .unwrap();

    println!("{}", format!("Tx: {}", tx).green());
}
