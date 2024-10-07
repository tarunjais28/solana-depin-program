use super::*;

pub async fn get_token_supplies(client: &RpcClient, tokens: &Tokens) {
    let token_a_supply = client
        .get_token_supply(&tokens.token_a)
        .expect("Error while getting Token A supply!");

    let token_b_supply = client
        .get_token_supply(&tokens.token_b)
        .expect("Error while getting Token B supply!");

    let token_c_supply = client
        .get_token_supply(&tokens.token_c)
        .expect("Error while getting Token C supply!");

    let dpit_supply = client
        .get_token_supply(&tokens.dpit)
        .expect("Error while getting DPIT supply!");

    println!("{}", format!("tokenA supply: {:#?}", token_a_supply).red());
    println!(
        "{}",
        format!("tokenB supply: {:#?}", token_b_supply).green()
    );
    println!("{}", format!("tokenC supply: {:#?}", token_c_supply).blue());
    println!("{}", format!("dpit supply: {:#?}", dpit_supply).purple());
}

pub async fn get_token_balances(client: &RpcClient, tokens: &Tokens, user: Pubkey) {
    let mut user_ata = get_associated_token_address(&user, &tokens.token_a);
    let mut balance = client
        .get_token_account(&user_ata)
        .expect("Error while getting Token A balance!")
        .unwrap();
    println!(
        "{}",
        format!("TokenA balance: {}", balance.token_amount.ui_amount_string).black()
    );

    user_ata = get_associated_token_address(&user, &tokens.token_b);
    balance = client
        .get_token_account(&user_ata)
        .expect("Error while getting Token B balance!")
        .unwrap();
    println!(
        "{}",
        format!("TokenB balance: {}", balance.token_amount.ui_amount_string).cyan()
    );

    user_ata = get_associated_token_address(&user, &tokens.token_c);
    balance = client
        .get_token_account(&user_ata)
        .expect("Error while getting Token C balance!")
        .unwrap();
    println!(
        "{}",
        format!("TokenC balance: {}", balance.token_amount.ui_amount_string).white()
    );

    user_ata = get_associated_token_address(&user, &tokens.dpit);
    balance = client
        .get_token_account(&user_ata)
        .expect("Error while getting Dpit balance!")
        .unwrap();
    println!(
        "{}",
        format!("Dpit balance: {}", balance.token_amount.ui_amount_string).bright_black()
    );
}

pub async fn get_program_data(client: &RpcClient, pda: &PdaAccounts) {
    let data: &[u8] = &client
        .get_account_data(&pda.global)
        .expect("Error while getting global data!");
    let global_data = get_deserialize_struct::<GlobalState>(data);
    println!(
        "{}",
        format!("Global State: {:#?}", global_data).bright_red()
    );

    let staked_amount = client
        .get_token_account_balance(&pda.escrow)
        .expect("Error while getting staked amount!");
    println!(
        "{}",
        format!("Staked Amount: {:#?}", staked_amount).bright_blue()
    );

    let token_a = client
        .get_token_account_balance(&pda.escrow_a)
        .expect("Error while getting escrow TokenA account!");
    println!("{}", format!("Escrow TokenA: {:#?}", token_a).bright_cyan());

    let token_b = client
        .get_token_account_balance(&pda.escrow_b)
        .expect("Error while getting escrow TokenB account!");
    println!(
        "{}",
        format!("Escrow TokenB: {:#?}", token_b).bright_green()
    );

    let token_c = client
        .get_token_account_balance(&pda.escrow_c)
        .expect("Error while getting escrow TokenC account!");
    println!(
        "{}",
        format!("Escrow TokenC: {:#?}", token_c).bright_magenta()
    );
}

pub async fn get_staked_data(client: &RpcClient, program_id: &Pubkey, user: &Pubkey) {
    let pda_staked_account = get_pda(
        &["lock".to_string().as_bytes(), &user.to_bytes()],
        program_id,
    );
    let data: &[u8] = &client
        .get_account_data(&pda_staked_account)
        .expect("Error while getting staked account data!");
    let stacked_account = get_deserialize_struct::<StakeState>(data);
    println!(
        "{}",
        format!("Staked State: {:#?}", stacked_account).bright_purple()
    );
}

pub async fn get_account_info(client: &RpcClient, user: &Pubkey) {
    // Fetch account information
    let account = client.get_account(&user).expect("Failed to get account");

    println!("{}", format!("Account data: {:#?}", account).green());
}

pub async fn get_pda_accounts(
    program: &anchor_client::Program<Rc<Keypair>>,
    pda: &PdaAccounts,
    user: &Pubkey,
) {
    let global_state: depin_program::depin_program::GlobalState =
        program.account(pda.global).await.unwrap();
    println!(
        "{}",
        format!("Global State: {:#?}", global_state).bright_red()
    );

    let pda_staked_account = get_pda(
        &["lock".to_string().as_bytes(), &user.to_bytes()],
        &depin_program::id(),
    );
    let stacked_account: depin_program::depin_program::StakeState =
        program.account(pda_staked_account).await.unwrap();
    println!(
        "{}",
        format!("Staked State: {:#?}", stacked_account).bright_purple()
    );
}
