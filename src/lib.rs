use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

/// This program extends the official Solana counter example
/// by adding decrement functionality and customizable step size
/// Original example: https://solana.com/ru/docs/programs/rust/program-structure


entrypoint!(process_instruction);

/// Main instruction processing function
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Deserialize instruction data into CounterInstruction enum
    let instruction = CounterInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    // Route to appropriate instruction handler
    match instruction {
        CounterInstruction::InitializeCounter { initial_value } => {
            process_initialize_counter(program_id, accounts, initial_value)?
        }
        CounterInstruction::IncrementCounter { step } => {
            process_increment_counter(program_id, accounts, step)?
        }
        CounterInstruction::DecrementCounter { step } => {
            process_decrement_counter(program_id, accounts, step)?
        }
    };

    Ok(())
}

/// Data structure stored in counter account
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    pub count: u64,
}

/// Available instructions for the counter program
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstruction {
    /// Initialize counter with starting value
    InitializeCounter { 
        initial_value: u64 
    },
    /// Increment counter by specified step (None = default step of 1)
    IncrementCounter {
        step: Option<u64>
    },
    /// Decrement counter by specified step (None = default step of 1)
    DecrementCounter {
        step: Option<u64>
    },
}

/// Initialize a new counter account with starting value
fn process_initialize_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    initial_value: u64,
) -> ProgramResult {
    msg!("Initializing counter with value: {}", initial_value);
    
    let accounts_iter = &mut accounts.iter();

    // Accounts expected in order:
    // 0. [writable] Counter account (to be created)
    // 1. [signer, writable] Payer account
    // 2. [] System program
    let counter_account = next_account_info(accounts_iter)?;
    let payer_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // Check if account is already initialized
    if counter_account.data.borrow().len() > 0 {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Calculate required account space and rent
    let account_space = std::mem::size_of::<CounterAccount>();
    let rent = Rent::get()?;
    let required_lamports = rent.minimum_balance(account_space);

    // Create counter account via system program
    invoke(
        &system_instruction::create_account(
            payer_account.key,
            counter_account.key,
            required_lamports,
            account_space as u64,
            program_id,
        ),
        &[
            payer_account.clone(),
            counter_account.clone(),
            system_program.clone(),
        ],
    )?;

    // Initialize counter data
    let counter_data = CounterAccount {
        count: initial_value,
    };

    // Serialize data into account
    let mut account_data = counter_account.data.borrow_mut();
    counter_data.serialize(&mut &mut account_data[..])?;

    msg!("Counter initialized successfully with value: {}", initial_value);
    Ok(())
}

/// Increment counter by specified step (default: 1)
fn process_increment_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    step: Option<u64>,
) -> ProgramResult {
    // Use default step of 1 if not specified
    let step_value = step.unwrap_or(1);
    msg!("Incrementing counter by: {}", step_value);
    
    let accounts_iter = &mut accounts.iter();
    
    // 0. [writable] Counter account
    let counter_account = next_account_info(accounts_iter)?;

    // Verify account ownership
    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if account is initialized
    if counter_account.data.borrow().len() == 0 {
        return Err(ProgramError::UninitializedAccount);
    }

    // Deserialize and update counter data
    let mut data = counter_account.data.borrow_mut();
    let mut counter_data = CounterAccount::try_from_slice(&data)?;

    // Safely increment counter with overflow check
    counter_data.count = counter_data
        .count
        .checked_add(step_value)
        .ok_or(ProgramError::InvalidAccountData)?;

    // Serialize updated data back to account
    counter_data.serialize(&mut &mut data[..])?;

    msg!("Counter incremented to: {}", counter_data.count);
    Ok(())
}

/// Decrement counter by specified step (default: 1)
fn process_decrement_counter(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    step: Option<u64>,
) -> ProgramResult {
    // Use default step of 1 if not specified
    let step_value = step.unwrap_or(1);
    msg!("Decrementing counter by: {}", step_value);
    
    let accounts_iter = &mut accounts.iter();
    
    // 0. [writable] Counter account
    let counter_account = next_account_info(accounts_iter)?;

    // Verify account ownership
    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Check if account is initialized
    if counter_account.data.borrow().len() == 0 {
        return Err(ProgramError::UninitializedAccount);
    }

    // Deserialize and update counter data
    let mut data = counter_account.data.borrow_mut();
    let mut counter_data = CounterAccount::try_from_slice(&data)?;

    // Safely decrement counter with underflow check
    counter_data.count = counter_data
        .count
        .checked_sub(step_value)
        .ok_or(ProgramError::InvalidAccountData)?;

    // Serialize updated data back to account
    counter_data.serialize(&mut &mut data[..])?;

    msg!("Counter decremented to: {}", counter_data.count);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use litesvm::LiteSVM;
    use solana_sdk::{
        account::ReadableAccount,
        instruction::{AccountMeta, Instruction},
        message::Message,
        signature::{Keypair, Signer},
        system_program,
        transaction::Transaction,
    };

    #[test]
    fn test_counter_program() {
        let mut svm = LiteSVM::new();

        let payer = Keypair::new();

        svm.airdrop(&payer.pubkey(), 1_000_000_000)
            .expect("Failed to airdrop");
        
        let program_keypair = Keypair::new();
        let program_id = program_keypair.pubkey();

        svm.add_program_from_file(
            program_id,
            "target/deploy/solana_counter_program.so"
        ).expect("Failed to load program");

        // Create new counter account with initial value
        let counter_keypair = Keypair::new();
        let initial_value: u64 = 42;

        println!("Testing counter initialization...");

        let init_instruction_data =
            borsh::to_vec(&CounterInstruction::InitializeCounter { initial_value })
                .expect("Failed to serialize instruction");

        let initialize_instruction = Instruction::new_with_bytes(
            program_id,
            &init_instruction_data,
            vec![
                AccountMeta::new(counter_keypair.pubkey(), true),
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );

        let message = Message::new(&[initialize_instruction], Some(&payer.pubkey()));
        let transaction = Transaction::new(
            &[&payer, &counter_keypair],
            message,
            svm.latest_blockhash()
        );

        let result = svm.send_transaction(transaction);
        assert!(result.is_ok(), "Initialize transaction should succeed");

        let logs = result.unwrap().logs;
        println!("Transaction logs:\n{:#?}", logs);  

        // Verify counter was initialized correctly
        let account = svm
            .get_account(&counter_keypair.pubkey())
            .expect("Failed to get counter account");

        let counter: CounterAccount = CounterAccount::try_from_slice(account.data())
            .expect("Failed to deserialize counter data");

        assert_eq!(counter.count, 42);
        println!("Counter initialized successfully with value: {}", counter.count);   
            
        // Test default increment (step = 1)
        println!("Testing default increment (step = 1)...");

        let increment_instruction_data =
            borsh::to_vec(&CounterInstruction::IncrementCounter { step: None })
                .expect("Failed to serialize instruction");

        let increment_instruction = Instruction::new_with_bytes(
            program_id,
            &increment_instruction_data,
            vec![AccountMeta::new(counter_keypair.pubkey(), false)],
        );

        let message = Message::new(&[increment_instruction], Some(&payer.pubkey()));
        let transaction = Transaction::new(
            &[&payer],
            message,
            svm.latest_blockhash()
        );

        let result = svm.send_transaction(transaction);
        assert!(result.is_ok(), "Increment transaction should succeed");

        let account = svm
            .get_account(&counter_keypair.pubkey())
            .expect("Failed to get counter account");

        let counter: CounterAccount = CounterAccount::try_from_slice(account.data())
            .expect("Failed to deserialize counter data");
        assert_eq!(counter.count, 43);
        println!("Counter incremented by 1 to: {}", counter.count);

        // Test custom increment (step = 5)
        println!("Testing custom increment (step = 5)...");

        let increment_by_5_instruction_data =
            borsh::to_vec(&CounterInstruction::IncrementCounter { step: Some(5) })
                .expect("Failed to serialize instruction");

        let increment_by_5_instruction = Instruction::new_with_bytes(
            program_id,
            &increment_by_5_instruction_data,
            vec![AccountMeta::new(counter_keypair.pubkey(), false)],
        );

        let message = Message::new(&[increment_by_5_instruction], Some(&payer.pubkey()));
        let transaction = Transaction::new(
            &[&payer],
            message,
            svm.latest_blockhash()
        );

        let result = svm.send_transaction(transaction);
        assert!(result.is_ok(), "Increment by 5 transaction should succeed");

        let account = svm
            .get_account(&counter_keypair.pubkey())
            .expect("Failed to get counter account");

        let counter: CounterAccount = CounterAccount::try_from_slice(account.data())
            .expect("Failed to deserialize counter data");
        assert_eq!(counter.count, 48);
        println!("Counter incremented by 5 to: {}", counter.count);

        // Test default decrement (step = 1)
        println!("Testing default decrement (step = 1)...");

        let decrement_instruction_data =
            borsh::to_vec(&CounterInstruction::DecrementCounter { step: None })
                .expect("Failed to serialize instruction");

        let decrement_instruction = Instruction::new_with_bytes(
            program_id,
            &decrement_instruction_data,
            vec![AccountMeta::new(counter_keypair.pubkey(), false)],
        );

        let message = Message::new(&[decrement_instruction], Some(&payer.pubkey()));
        let transaction = Transaction::new(
            &[&payer],
            message,
            svm.latest_blockhash()
        );

        let result = svm.send_transaction(transaction);
        assert!(result.is_ok(), "Decrement transaction should succeed");

        let account = svm
            .get_account(&counter_keypair.pubkey())
            .expect("Failed to get counter account");

        let counter: CounterAccount = CounterAccount::try_from_slice(account.data())
            .expect("Failed to deserialize counter data");
        assert_eq!(counter.count, 47);
        println!("Counter decremented by 1 to: {}", counter.count);

        // Test custom decrement (step = 3)
        println!("Testing custom decrement (step = 3)...");

        let decrement_by_3_instruction_data =
            borsh::to_vec(&CounterInstruction::DecrementCounter { step: Some(3) })
                .expect("Failed to serialize instruction");

        let decrement_by_3_instruction = Instruction::new_with_bytes(
            program_id,
            &decrement_by_3_instruction_data,
            vec![AccountMeta::new(counter_keypair.pubkey(), false)],
        );

        let message = Message::new(&[decrement_by_3_instruction], Some(&payer.pubkey()));
        let transaction = Transaction::new(
            &[&payer],
            message,
            svm.latest_blockhash()
        );

        let result = svm.send_transaction(transaction);
        assert!(result.is_ok(), "Decrement by 3 transaction should succeed");

        let account = svm
            .get_account(&counter_keypair.pubkey())
            .expect("Failed to get counter account");

        let counter: CounterAccount = CounterAccount::try_from_slice(account.data())
            .expect("Failed to deserialize counter data");
        assert_eq!(counter.count, 44);
        println!("Counter decremented by 3 to: {}", counter.count);

        // Test underflow protection
        println!("Testing underflow protection...");

        // Reset counter to 0
        let reset_instruction_data =
            borsh::to_vec(&CounterInstruction::DecrementCounter { step: Some(44) })
                .expect("Failed to serialize instruction");

        let reset_instruction = Instruction::new_with_bytes(
            program_id,
            &reset_instruction_data,
            vec![AccountMeta::new(counter_keypair.pubkey(), false)],
        );

        let message = Message::new(&[reset_instruction], Some(&payer.pubkey()));
        let transaction = Transaction::new(
            &[&payer],
            message,
            svm.latest_blockhash()
        );

        let result = svm.send_transaction(transaction);
        assert!(result.is_ok(), "Reset transaction should succeed");

        // Attempt to decrement below 0 should fail
        let decrement_below_zero_instruction_data =
            borsh::to_vec(&CounterInstruction::DecrementCounter { step: Some(1) })
                .expect("Failed to serialize instruction");

        let decrement_below_zero_instruction = Instruction::new_with_bytes(
            program_id,
            &decrement_below_zero_instruction_data,
            vec![AccountMeta::new(counter_keypair.pubkey(), false)],
        );

        let message = Message::new(&[decrement_below_zero_instruction], Some(&payer.pubkey()));
        let transaction = Transaction::new(
            &[&payer],
            message,
            svm.latest_blockhash()
        );

        let result = svm.send_transaction(transaction);
        assert!(result.is_err(), "Decrement below zero should fail");
        println!("Underflow protection test passed!");
    }
}
