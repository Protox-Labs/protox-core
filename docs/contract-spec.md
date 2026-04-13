# Protox Core Contract Specification

This document details the public API and data structures for the Protox Core vault contract.

## Contract Interface

The vault contract is a Soroban smart contract implemented in Rust. It interacts with the [Stellar Asset Contract](https://soroban.stellar.org/docs/reference/interfaces/asset-contract) (SAC) for token transfers.

### Public Methods

#### `initialize(e: Env, admin: Address, token: Address) -> Result<(), VaultError>`
Initializes the vault with an admin and a token address.
- **admin**: The address with administrative privileges (e.g., reward distribution).
- **token**: The address of the token asset the vault accepts for deposits.
- **Returns**: `Ok(())` or `AlreadyInitialized` error.

#### `deposit(e: Env, user: Address, amount: i128) -> Result<(), VaultError>`
Deposits tokens into the vault and updates reward debt.
- **user**: The address of the depositor.
- **amount**: The quantity of tokens to deposit.
- **Returns**: `Ok(())`, `NegativeAmount`, or `NotInitialized` error.

#### `withdraw(e: Env, user: Address, amount: i128) -> Result<(), VaultError>`
Withdraws tokens from the vault and updates reward debt.
- **user**: The address of the withdrawer.
- **amount**: The quantity of tokens to withdraw.
- **Returns**: `Ok(())`, `NegativeAmount`, `InsufficientBalance`, or `NotInitialized` error.

#### `distribute_rewards(e: Env, admin: Address, amount: i128) -> Result<(), VaultError>`
Distributes rewards to all depositors by updating the `RewardPerShare` index.
- **admin**: The address of the vault administrator.
- **amount**: The quantity of reward tokens to distribute.
- **Returns**: `Ok(())`, `Unauthorized`, `NegativeAmount`, or `NotInitialized` error.

#### `calculate_pending_reward(e: &Env, user: &Address) -> Result<i128, VaultError>`
Calculates the current pending rewards for a specific user.
- **user**: The address of the depositor.
- **Returns**: The amount of pending rewards.

#### `get_balance(e: Env, user: Address) -> i128`
Returns the user's current share balance in the vault.

#### `get_total_shares(e: Env) -> i128`
Returns the total number of shares issued by the vault.

## Error Codes

| Error Name | Code | Description |
|---|---|---|
| `NotInitialized` | 1 | Contract hasn't been initialized. |
| `AlreadyInitialized` | 2 | Contract has already been initialized. |
| `NegativeAmount` | 3 | Provided amount must be positive. |
| `InsufficientBalance` | 4 | User has insufficient shares for withdrawal. |
| `Unauthorized` | 5 | Caller doesn't have required permissions. |
| `InvalidToken` | 6 | Provided token address is invalid. |
| `RewardCalculationError` | 7 | Error calculating rewards. |

## Event Specification

### `initialized`
- **Topics**: `["initialized", admin: Address]`
- **Data**: `token: Address`

### `deposit`
- **Topics**: `["deposit", user: Address]`
- **Data**: `amount: i128`

### `withdraw`
- **Topics**: `["withdraw", user: Address]`
- **Data**: `amount: i128`

### `reward`
- **Topics**: `["reward"]`
- **Data**: `amount: i128`
