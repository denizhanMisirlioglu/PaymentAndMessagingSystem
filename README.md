# Payment and Messaging System

This project is a **Rust-based** Stellar payment and messaging system built with the **Soroban SDK**. The system allows users to send payments with attached messages, handle multi-recipient payments, and check account balances.

## Features

- **Send Payment**: Users can send payments along with a message to a recipient.
- **Send Bulk Payments**: Users can send payments to multiple recipients simultaneously.
- **Check Balance**: Query the balance of an account on the Stellar network.

## How It Works

- Built using the **Soroban SDK** for Stellar smart contracts.
- Contracts are deployed and executed on the Stellar blockchain.
- Tokens can be transferred between accounts with optional messages attached.

## Project Structure

- **src/lib.rs**: Contains the main contract code which defines the functions for payment, bulk payment, and balance checking.
- **Cargo.toml**: Defines the project dependencies, including the Soroban SDK and the necessary libraries.

## Usage

1. **Send Payment**: Transfers tokens to a recipient with a message.
2. **Send Multiple Payments**: Distribute tokens to multiple addresses in a single transaction.
3. **Check Balance**: Retrieve the token balance of any Stellar address.

## Build and Deployment

- Compile the project to WebAssembly:
  ```sh
  cargo build --target wasm32-unknown-unknown --release
