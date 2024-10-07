use solana_client::rpc_config::RpcTransactionLogsConfig;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedConfirmedBlock, EncodedTransactionWithStatusMeta,
};
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};
use std::str::FromStr;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Initialize Solana RPC Client
    let rpc_url = "https://api.devnet.solana.com"; // Use your preferred cluster (Devnet, Mainnet, Testnet)
    let client = RpcClient::new(rpc_url.to_string());

    // Public key of the program you want to listen to
    let program_id = Pubkey::from_str("3W7pnY6U3Aa7ERYf7KTwMmfNmyFRNTNivR4Ya6nKScXh")
        .expect("Invalid program ID");

    // Start listening for events
    listen_for_events(client, program_id).await;
}

async fn listen_for_events(client: RpcClient, program_id: Pubkey) {
    let mut last_slot: u64 = 0;
    let config = RpcBlockConfig {
        encoding: Some(UiTransactionEncoding::Base58),
        transaction_details: Some(TransactionDetails::Full),
        rewards: None,
        commitment: None,
        max_supported_transaction_version: Some(2),
    };

    loop {
        // Poll for the most recent slot (block height)
        match client.get_slot() {
            Ok(current_slot) => {
                if current_slot > last_slot {
                    // Fetch transactions for the new slot
                    let confirmed_block = client.get_block_with_config(current_slot, config);
                    if let Ok(block) = confirmed_block {
                        if let Some(tx) = block.transactions {
                            process_block_logs(tx, &program_id);
                        }
                    }
                    last_slot = current_slot;
                }
            }
            Err(err) => {
                eprintln!("Error getting slot: {}", err);
            }
        }

        // Sleep before polling again (adjust the duration as needed)
        sleep(Duration::from_millis(100)).await;
    }
}

fn process_block_logs(transactions: Vec<EncodedTransactionWithStatusMeta>, program_id: &Pubkey) {
    for transaction in transactions {
        if let Some(meta) = transaction.meta {
            if let OptionSerializer::Some(log_messages) = meta.log_messages {
                for log in log_messages {
                    // println!("{}", log);
                    // Filter logs by the program ID
                    if log.contains(&program_id.to_string()) {
                        handle_event(log);
                    }
                }
            }
        }
    }
}

// Function to handle event when a log matches the program
fn handle_event(log: String) {
    // Parse and process the event
    println!("Processing event: {}", log);

    // Implement your event handling logic here, e.g., call another service, store data, etc.
}
