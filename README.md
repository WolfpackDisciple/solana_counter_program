# Solana Counter Program

An enhanced Solana counter program example featuring increment/decrement functionality 
with customizable step sizes. This program extends the official Solana counter example 
with additional capabilities.
You can read more here: https://solana.com/ru/docs/programs/rust/program-structure
## Features

- **Initialize Counter**: Create a new counter with a custom initial value
- **Increment Counter**: Increase counter value by specified step (default: 1)
- **Decrement Counter**: Decrease counter value by specified step (default: 1) 
- **Overflow/Underflow Protection**: Safe arithmetic operations with proper error
- handling
- **Custom Step Sizes**: Optional parameter to specify custom increment/decrement steps

## Prerequisites

- Rust 1.60.0 or later
- Solana CLI tools 1.10.0 or later
- Git

## Quick Start

### 1. Install Dependencies

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.17.24/install)"

# Verify installations
rustc --version
solana --version
```

### 2. Clone and Setup Project

```bash
# Clone the repository
git clone https://github.com/WolfpackDisciple/solana_counter_program.git
cd solana_counter_program

# Build the program
cargo build-sbf

# Set Solana CLI to localnet
solana config set --url localhost
```

### 3. Cargo.toml Configuration

Ensure your `Cargo.toml` contains the following dependencies:

```toml
[package]
name = "counter_program"
version = "0.1.0"
description = "Enhanced Solana counter program with increment/decrement functionality"
edition = "2021"

[lib]
crate-type = ["cdylib", "lib"]

[dependencies]
solana-program = "2.2.0"
borsh = { version = "0.10.3", features = ["derive"] }

[dev-dependencies]
litesvm = "0.6.1"
solana-sdk = "2.2.0"

[[example]]
name = "client"
path = "examples/client.rs"
```

### 4. Build and Deploy

```bash
# Build the program
cargo build-sbf

# Start local validator (in separate terminal)
solana-test-validator

# Deploy program
solana program deploy ./target/deploy/counter_program.so

# Get program ID
solana address -k ./target/deploy/counter_program-keypair.json
```

### 5. Run Tests

```bash
# Run unit tests
cargo test -- --nocapture

# Run integration tests
cargo test --test integration -- --nocapture
```

### 6. Run Client Example

```bash
# Update program ID in examples/client.rs first
# Replace with your actual program ID from deployment
cargo run --example client
```

## Program Structure

### Instructions

The program supports three instructions:

1. **InitializeCounter**: Creates a new counter account
   - `initial_value: u64` - Starting value for the counter

2. **IncrementCounter**: Increases counter value
   - `step: Option<u64>` - Optional step size (default: 1)

3. **DecrementCounter**: Decreases counter value  
   - `step: Option<u64>` - Optional step size (default: 1)

### Account Structure

```rust
pub struct CounterAccount {
    pub count: u64,  // Current counter value
}
```

## Usage Examples

### Initialize Counter
```rust
// Initialize with value 100
CounterInstruction::InitializeCounter { initial_value: 100 }
```

### Increment Operations
```rust
// Increment by default step (1)
CounterInstruction::IncrementCounter { step: None }

// Increment by custom step (5)  
CounterInstruction::IncrementCounter { step: Some(5) }
```

### Decrement Operations
```rust
// Decrement by default step (1)
CounterInstruction::DecrementCounter { step: None }

// Decrement by custom step (3)
CounterInstruction::DecrementCounter { step: Some(3) }
```

## Security Features

- **Ownership Verification**: Ensures only program-owned accounts can be modified
- **Initialization Checks**: Prevents re-initialization of existing accounts
- **Arithmetic Safety**: Protected against overflow/underflow attacks
- **Account Validation**: Proper validation of all input accounts

## Testing

The project includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_counter_program -- --nocapture
```

## Project Structure

```
solana_counter_program/
├── src/
│   └── lib.rs          # Main program logic
├── examples/
│   └── client.rs       # Example client implementation
├── target/
│   └── deploy/
│       ├── counter_program.so           # Compiled program
│       └── counter_program-keypair.json # Program keypair
├── Cargo.toml          # Project configuration
└── README.md           # This file
```

## Documentation

This program extends the official Solana counter example with enhanced functionality. 
For detailed explanations of Solana program structure and concepts, 
refer to the official documentation:

- [Solana Documentation](https://solana.com/docs)
- [Solana Rust SDK](https://docs.rs/solana-program/latest/solana_program/)
- [BPF Program Development](https://solana.com/docs/programs/rust)

## Contributing

This is an educational project based on Solana's official examples. Contributions
that improve security, add features, or enhance documentation are welcome.

## License

This project is open source and available under the MIT License.

## Disclaimer

This code is based on official Solana examples and documentation. 
It is intended for educational purposes and should be thoroughly audited 
before use in production environments.
