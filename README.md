Solana Counter Program
 
An enhanced Solana counter program example featuring increment/decrement functionality with customizable step sizes. This program extends the official Solana counter example with additional capabilities. You can read more here: https://solana.com/ru/docs/programs/rust/program-structure

⚠️ Disclaimer: This is an educational project. The code is based on official Solana examples and documentation. It is intended for learning purposes and has not been audited for security. It should not be used in production environments.
Features

    Initialize Counter: Create a new counter with a custom initial value

    Increment Counter: Increase counter value by specified step (default: 1)

    Decrement Counter: Decrease counter value by specified step (default: 1)

    Custom Step Sizes: Optional parameter to specify custom increment/decrement steps

Prerequisites

    Rust 1.60.0 or later

    Solana CLI tools 1.10.0 or later

    Git

Quick Start
1. Install Dependencies
bash

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
Solana Counter Program

An enhanced Solana counter program example featuring increment/decrement functionality with customizable step sizes. This program extends the official Solana counter example with additional capabilities. You can read more here: https://solana.com/ru/docs/programs/rust/program-structure

⚠️ Disclaimer: This is an educational project. The code is based on official Solana examples and documentation. It is intended for learning purposes and has not been audited for security. It should not be used in production environments. Features

Initialize Counter: Create a new counter with a custom initial value

Increment Counter: Increase counter value by specified step (default: 1)

Decrement Counter: Decrease counter value by specified step (default: 1)

Custom Step Sizes: Optional parameter to specify custom increment/decrement steps

Prerequisites

Rust 1.60.0 or later

Solana CLI tools 1.10.0 or later

Git

Quick Start

    Install Dependencies bash

Install Rust

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh source ~/.cargo/env
Install Solana CLI

sh -c "$(curl -sSfL https://release.solana.com/v1.17.24/install)"
Verify installations

rustc --version solana --version

    Clone and Setup Project bash

Clone the repository

git clone https://github.com/WolfpackDisciple/solana_counter_program.git cd solana_counter_program
Build the program

cargo build-sbf
Set Solana CLI to localnet

solana config set --url localhost

    Cargo.toml Configuration



[package] name = "solana_counter_program" 
version = "0.1.0" edition = "2021"

[lib] crate-type = ["cdylib", "lib"]

[dependencies] borsh = "1.5.7" 

solana-program = "2.2.0"

[dev-dependencies] 
litesvm = "0.6.1"  
solana-client = "2.2.0" 
solana-sdk = "2.2.0"
tokio = "1.47.1" 

[[example]] 
name = "client"
path = "examples/client.rs" 

    Build and Deploy bash

Build the program

cargo build-sbf
Start local validator (in separate terminal)

solana-test-validator
Deploy program (используйте правильное имя файла!)

solana program deploy ./target/deploy/solana_counter_program.so
Get program ID

solana address -k ./target/deploy/solana_counter_program-keypair.json

    Run Tests bash

Run unit tests (используют LiteSVM)

cargo test -- --nocapture
Run the client example (предварительно обновив program_id в client.rs)

cargo run --example client

Program Structure Instructions

The program supports three instructions:

InitializeCounter: Creates a new counter account

    initial_value: u64 - Starting value for the counter

IncrementCounter: Increases counter value

    step: Option<u64> - Optional step size (default: 1)

DecrementCounter: Decreases counter value

    step: Option<u64> - Optional step size (default: 1)

Usage Examples

Initialize Counter rust

// Initialize with value 100 CounterInstruction::InitializeCounter { initial_value: 100 }

Increment Operations rust

// Increment by default step (1) CounterInstruction::IncrementCounter { step: None }

// Increment by custom step (5)
CounterInstruction::IncrementCounter { step: Some(5) }

Decrement Operations rust

// Decrement by default step (1) CounterInstruction::DecrementCounter { step: None }

// Decrement by custom step (3) CounterInstruction::DecrementCounter { step: Some(3) }

Testing

The project includes comprehensive tests: bash
Run all tests

cargo test
Run with verbose output

cargo test -- --nocapture
Run specific test

cargo test test_counter_program -- --nocapture

Project Structure text

solana_counter_program/ ├── src/ │ └── lib.rs # Main program logic ├── examples/ │ └── client.rs # Example client implementation ├── target/ │ └── deploy/ │ ├── solana_counter_program.so # Compiled program │ └── solana_counter_program-keypair.json # Program keypair ├── Cargo.toml # Project configuration └── README.md # This file

Documentation

This program extends the official Solana counter example with enhanced functionality. For detailed explanations of Solana program structure and concepts, refer to the official documentation: Solana Rust Program Structure Contributing

This is an educational project based on Solana's official examples. Contributions that improve security, add features, or enhance documentation are welcome. License

This project is open source and available under the MIT License.
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.17.24/install)"

# Verify installations
rustc --version
solana --version

2. Clone and Setup Project
bash

# Clone the repository
git clone https://github.com/WolfpackDisciple/solana_counter_program.git
cd solana_counter_program

# Build the program
cargo build-sbf

# Set Solana CLI to localnet
solana config set --url localhost

3. Cargo.toml Configuration



[package]
name = "solana_counter_program" 
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]

[dependencies]
borsh = "1.5.7" 
solana-program = "2.2.0"

[dev-dependencies]
litesvm = "0.6.1" 
solana-client = "2.2.0" 
solana-sdk = "2.2.0"
tokio = "1.47.1"

[[example]]
name = "client"
path = "examples/client.rs" 

4. Build and Deploy
bash

# Build the program
cargo build-sbf

# Start local validator (in separate terminal)
solana-test-validator

# Deploy program 
solana program deploy ./target/deploy/solana_counter_program.so

# Get program ID
solana address -k ./target/deploy/solana_counter_program-keypair.json

5. Run Tests
bash

# Run unit tests (use LiteSVM)
cargo test -- --nocapture

# Run the client example (update program_id in client.rs)
cargo run --example client

Program Structure
Instructions

The program supports three instructions:

    InitializeCounter: Creates a new counter account

        initial_value: u64 - Starting value for the counter

    IncrementCounter: Increases counter value

        step: Option<u64> - Optional step size (default: 1)

    DecrementCounter: Decreases counter value

        step: Option<u64> - Optional step size (default: 1)

Usage Examples

Initialize Counter
rust

// Initialize with value 100
CounterInstruction::InitializeCounter { initial_value: 100 }

Increment Operations
rust

// Increment by default step (1)
CounterInstruction::IncrementCounter { step: None }

// Increment by custom step (5)  
CounterInstruction::IncrementCounter { step: Some(5) }

Decrement Operations
rust

// Decrement by default step (1)
CounterInstruction::DecrementCounter { step: None }

// Decrement by custom step (3)
CounterInstruction::DecrementCounter { step: Some(3) }

Testing

The project includes comprehensive tests:
bash

# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_counter_program -- --nocapture

        # This file

Documentation

This program extends the official Solana counter example with enhanced functionality. For detailed explanations of Solana program structure and concepts, refer to the official documentation: Solana Rust Program Structure
Contributing

This is an educational project based on Solana's official examples. Contributions that improve security, add features, or enhance documentation are welcome.
License

This project is open source and available under the MIT License.
