use borsh::{BorshDeserialize, BorshSerialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_program,
    transaction::Transaction,
};
use std::str::FromStr;
use counter_program::CounterInstruction;

#[tokio::main]
async fn main() {
    // Replace with your actual program ID from deployment
    let program_id = Pubkey::from_str("3T8DsLJF1UYYq6zzaVrZTPmEckZktdY5dxHWHWeJVS6r")
        .expect("Invalid program ID");

    // Connect to local Solana cluster (devnet or testnet)
    let rpc_url = String::from("http://localhost:8899");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Generate keypair for transaction fees
    let payer = Keypair::new();

    // Request airdrop for transaction fees (devnet/testnet only)
    println!("Requesting airdrop...");
    let airdrop_signature = client
        .request_airdrop(&payer.pubkey(), 1_000_000_000) // 1 SOL
        .expect("Failed to request airdrop");

    // Wait for airdrop confirmation
    println!("Waiting for airdrop confirmation...");
    loop {
        if client
            .confirm_transaction(&airdrop_signature)
            .unwrap_or(false)
        {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    println!("Airdrop confirmed!");

    // --- Initialize Counter ---
    println!("\n1. Initializing counter...");
    let counter_keypair = Keypair::new();
    let initial_value = 100u64;

    // Create initialize instruction
    let init_instruction_data = borsh::to_vec(&CounterInstruction::InitializeCounter { 
        initial_value 
    }).expect("Failed to serialize instruction");

    let initialize_instruction = Instruction::new_with_bytes(
        program_id,
        &init_instruction_data,
        vec![
            AccountMeta::new(counter_keypair.pubkey(), true),     // New counter account (signer)
            AccountMeta::new(payer.pubkey(), true),               // Payer account (signer)
            AccountMeta::new_readonly(system_program::id(), false), // System program
        ],
    );

    // Create and send transaction
    let mut transaction = Transaction::new_with_payer(
        &[initialize_instruction], 
        Some(&payer.pubkey())
    );

    let blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");
    
    transaction.sign(&[&payer, &counter_keypair], blockhash);

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("✅ Counter initialized successfully!");
            println!("   Transaction: {}", signature);
            println!("   Counter address: {}", counter_keypair.pubkey());
            println!("   Initial value: {}", initial_value);
        }
        Err(err) => {
            eprintln!("❌ Failed to initialize counter: {}", err);
            return;
        }
    }

    // --- Increment Counter (default step = 1) ---
    println!("\n2. Incrementing counter by default step (1)...");
    
    let increment_data = borsh::to_vec(&CounterInstruction::IncrementCounter { 
        step: None  // Default step of 1
    }).expect("Failed to serialize instruction");

    let increment_instruction = Instruction::new_with_bytes(
        program_id,
        &increment_data,
        vec![AccountMeta::new(counter_keypair.pubkey(), false)], // Writable, not signer
    );

    let mut transaction = Transaction::new_with_payer(
        &[increment_instruction], 
        Some(&payer.pubkey())
    );

    let blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");
    transaction.sign(&[&payer], blockhash);

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("✅ Counter incremented by 1!");
            println!("   Transaction: {}", signature);
        }
        Err(err) => {
            eprintln!("❌ Failed to increment counter: {}", err);
        }
    }

    // --- Increment Counter (custom step = 5) ---
    println!("\n3. Incrementing counter by custom step (5)...");
    
    let increment_by_5_data = borsh::to_vec(&CounterInstruction::IncrementCounter { 
        step: Some(5)  // Custom step of 5
    }).expect("Failed to serialize instruction");

    let increment_by_5_instruction = Instruction::new_with_bytes(
        program_id,
        &increment_by_5_data,
        vec![AccountMeta::new(counter_keypair.pubkey(), false)],
    );

    let mut transaction = Transaction::new_with_payer(
        &[increment_by_5_instruction], 
        Some(&payer.pubkey())
    );

    let blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");
    transaction.sign(&[&payer], blockhash);

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("✅ Counter incremented by 5!");
            println!("   Transaction: {}", signature);
        }
        Err(err) => {
            eprintln!("❌ Failed to increment counter by 5: {}", err);
        }
    }

    // --- Decrement Counter (default step = 1) ---
    println!("\n4. Decrementing counter by default step (1)...");
    
    let decrement_data = borsh::to_vec(&CounterInstruction::DecrementCounter { 
        step: None  // Default step of 1
    }).expect("Failed to serialize instruction");

    let decrement_instruction = Instruction::new_with_bytes(
        program_id,
        &decrement_data,
        vec![AccountMeta::new(counter_keypair.pubkey(), false)],
    );

    let mut transaction = Transaction::new_with_payer(
        &[decrement_instruction], 
        Some(&payer.pubkey())
    );

    let blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");
    transaction.sign(&[&payer], blockhash);

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("✅ Counter decremented by 1!");
            println!("   Transaction: {}", signature);
        }
        Err(err) => {
            eprintln!("❌ Failed to decrement counter: {}", err);
        }
    }

    // --- Decrement Counter (custom step = 3) ---
    println!("\n5. Decrementing counter by custom step (3)...");
    
    let decrement_by_3_data = borsh::to_vec(&CounterInstruction::DecrementCounter { 
        step: Some(3)  // Custom step of 3
    }).expect("Failed to serialize instruction");

    let decrement_by_3_instruction = Instruction::new_with_bytes(
        program_id,
        &decrement_by_3_data,
        vec![AccountMeta::new(counter_keypair.pubkey(), false)],
    );

    let mut transaction = Transaction::new_with_payer(
        &[decrement_by_3_instruction], 
        Some(&payer.pubkey())
    );

    let blockhash = client.get_latest_blockhash().expect("Failed to get blockhash");
    transaction.sign(&[&payer], blockhash);

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("✅ Counter decremented by 3!");
            println!("   Transaction: {}", signature);
        }
        Err(err) => {
            eprintln!("❌ Failed to decrement counter by 3: {}", err);
        }
    }

    // --- Read Final Counter Value ---
    println!("\n6. Reading final counter value...");
    
    match client.get_account_data(&counter_keypair.pubkey()) {
        Ok(account_data) => {
            let counter = counter_program::CounterAccount::try_from_slice(&account_data)
                .expect("Failed to deserialize counter data");
            
            println!("📊 Final counter value: {}", counter.count);
            println!("🎉 All operations completed successfully!");
        }
        Err(err) => {
            eprintln!("❌ Failed to read counter value: {}", err);
        }
    }
}