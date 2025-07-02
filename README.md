## Solidity and Rust
this is Ethereum and Solana program.
# 1. Ethereum 
## SimpleWallet Smart Contract

This is a basic Ethereum smart contract written in Solidity that allows:

- Users to deposit ETH
- The contract **owner** to withdraw ETH
- Tracking of individual balances
- Logging of deposits using a custom event

## Features

- ✅ Accepts ETH deposits from any address  
- 🔐 Only the contract owner can withdraw funds  
- 📊 Tracks balances per user  
- 📢 Emits a `Deposited` event on each deposit  

## Contract Details

- **Contract Name:** `SimpleWallet`
- **Solidity Version:** `^0.8.0`
- **License:** MIT

## Functions

### `constructor()`
Sets the deployer as the `owner`.

### `deposit() external payable`
- Accepts ETH deposits from users.
- Updates the sender's balance.
- Emits a `Deposited(address indexed sender, uint256 amount)` event.

### `withdraw(uint256 amount) external`
- Only callable by the `owner`.
- Withdraws a specified amount of ETH to the owner’s address.

### `getContractBalance() external view returns (uint256)`
- Returns the total ETH held in the contract.

## Event

```solidity
event Deposited(address indexed sender, uint256 amount);

```
# 2. Solana 
## 🔢 Solana Number Store Program

This Solana program, written in Rust using the [Anchor framework](https://www.anchor-lang.com/), allows a user to:

- Initialize an account that stores a number on-chain
- Update the number, only if the signer is the original authority

---

## 📁 Project Structure

```bash
solana_number_store/
├── programs/
│   └── solana_number_store/
│       └── src/
│           └── lib.rs      # Main Solana program
├── tests/
│   └── solana_number_store.ts (optional test)
├── migrations/
├── Anchor.toml
├── Cargo.toml
└── README.md
```
## Build the Program
```bash
anchor build
```
##  Deploy to Devnet
```bash
anchor deploy --provider.cluster devnet
```
