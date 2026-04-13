# Protox Core Architecture

Protox Core is designed as a modular, high-performance vault and reward distribution protocol for the Stellar network using Soroban.

## Core Components

The vault protocol consists of several key modules:
- **Vault Contract**: The main entry point for deposits, withdrawals, and reward management.
- **Storage Module**: Defines how data is persisted on the ledger using Soroban's `DataKey` system.
- **Event System**: Standardized event emission for all state-changing actions.
- **Error Handling**: Custom error types for clear contract reverts and client-side debugging.

## Storage Design

Protox Core uses a structured storage approach:
- **Instance Storage**: For global parameters like `Admin`, `Token`, `TotalShares`, and `RewardPerShare`.
- **Persistent Storage**: For user-specific data like `Balance` and `UserRewardDebt`.

### Key Data Structures

```rust
pub enum DataKey {
    Admin,              // The vault administrator
    Token,              // The underlying asset token
    Balance(Address),   // User-specific vault shares
    TotalShares,        // Total shares across all users
    RewardPerShare,     // Accumulated reward index
    UserRewardDebt(Address), // Tracking user-specific reward payouts
}
```

## Reward Distribution Model

The vault uses a scalable reward distribution algorithm (inspired by Synthetix) to avoid O(n) loops.
1. **Reward-per-Share Index**: When rewards are distributed, we update a global index:
   `RewardPerShare += RewardAmount * PRECISION / TotalShares`
2. **User Reward Debt**: When a user deposits or withdraws, we update their `UserRewardDebt`:
   `UserRewardDebt = UserBalance * RewardPerShare / PRECISION`
3. **Pending Rewards**: A user's accumulated rewards can be calculated on-the-fly:
   `PendingRewards = (UserBalance * RewardPerShare / PRECISION) - UserRewardDebt`

## Modular Code Organization

- **lib.rs**: Defines the `VaultContract` struct and its public implementation.
- **storage.rs**: Contains all getter/setter logic for the contract's state.
- **events.rs**: Centralized logic for event publishing.
- **errors.rs**: Custom `#[contracterror]` enum.

## Security Considerations

- **Authorization**: All state-changing actions (deposit, withdraw, distribute_rewards) require proper `Address.require_auth()`.
- **Integrity**: Vault shares are always backed 1:1 by the underlying token in the contract's balance.
- **Overflow Protection**: Rust's built-in overflow checks and Soroban's `i128` type ensure numerical safety.

## Future Enhancements (TODOs)

- **Upgradeable Pattern**: Transitioning to a proxy-based or contract-ID-based upgrade system.
- **Advanced Governance**: Implementing multi-sig or DAO-based controls for admin actions.
- **Yield Strategies**: Adding the ability to deploy vault liquidity into other Stellar protocols.
