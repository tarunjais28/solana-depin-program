use anchor_client::{
    anchor_lang::prelude::*,
    solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig},
    solana_sdk::{
        signature::{read_keypair_file, Keypair},
        signer::Signer,
        system_program,
        {commitment_config::CommitmentConfig, system_instruction, transaction::Transaction},
    },
    Client, Cluster,
};
use anchor_spl::{
    associated_token::{self, get_associated_token_address},
    token::spl_token,
};
use colored::Colorize;
use std::{rc::Rc, str::FromStr};

mod execute;
mod helper;
mod query;
mod structs;

pub use crate::{execute::*, helper::*, query::*, structs::*};

#[tokio::main]
async fn main() {
    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url);

    let program_id = depin_program::id();
    let tokens = Tokens::load();
    let user_keypair =
        read_keypair_file("/Users/tarunjaiswal/.config/solana/id.json").expect("Keypair not found");
    let user = user_keypair.pubkey();

    // get_account_info(&client, &user).await;

    // // Interact with the program
    // get_token_supplies(&client, &tokens).await;

    let pda_accounts = PdaAccounts::load(&program_id);
    // get_program_data(&client, &pda_accounts).await;

    // get_staked_data(&client, &program_id, &user).await;
    let rc_client = Client::new(Cluster::Devnet, Rc::new(user_keypair));
    let program = rc_client.program(depin_program::ID).unwrap();
    mint_tokens(&program, &pda_accounts, &tokens, &user).await;
    // burn_tokens(&program, &pda_accounts, &tokens, &user).await;
    // get_token_balances(&client, &tokens, user).await;
    // stake(&program, &pda_accounts, &tokens, &user).await;
    // unstake(&program, &pda_accounts, &tokens, &user).await;
    // get_pda_accounts(&program, &pda_accounts, &user).await;
}
