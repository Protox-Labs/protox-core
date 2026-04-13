# Protox Core

Protox Core is a high-performance vault and reward distribution protocol built on the Stellar network using Soroban. It provides a modular, secure, and efficient way for users to deposit tokens, track balances, and receive proportional reward distributions.

## Features

- **Vault Deposits**: Securely deposit tokens and receive vault shares.
- **Balance Tracking**: Real-time tracking of user balances and total vault liquidity.
- **Reward Distribution**: Pro-rata reward distribution based on vault shares.
- **Modular Design**: Separated concerns for storage, events, and error handling.
- **Developer-First**: Extensively documented and designed for open-source contributions.

## Architecture

Protox Core is built as a set of Soroban smart contracts. The core vault contract manages:
- **Token Interactions**: Integration with Stellar's Asset Contract interface.
- **Share Accounting**: Maintaining accurate ledger entries for each depositor.
- **Reward Logic**: Scalable reward calculation using a "Reward-per-Share" pattern.

See [architecture.md](docs/architecture.md) for a deep dive into the protocol design.

## Project Structure

```text
protox-core/
├── contracts/
│   └── vault-contract/      # Core vault smart contract
│       ├── src/             # Contract source code
│       └── Cargo.toml       # Rust dependencies
├── scripts/                 # Deployment and initialization scripts
├── tests/                   # Integration and end-to-end tests
├── docs/                    # Technical documentation
└── .github/                 # GitHub templates and workflows
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
- [Node.js](https://nodejs.org/)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/protox/protox-core.git
   cd protox-core
   ```

2. Build the contract:
   ```bash
   cd contracts/vault-contract
   cargo build --target wasm32-unknown-unknown --release
   ```

3. Run unit tests:
   ```bash
   cargo test
   ```

## Deployment

Refer to the [scripts/](scripts/) directory for deployment and initialization examples using TypeScript.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to get involved. 

**Contribution Areas for Newcomers:**
- Implement reward claiming logic.
- Optimize storage gas costs.
- Add support for multiple reward tokens.
- Improve security validation checks.

## License

Protox Core is open-source software licensed under the [MIT License](LICENSE).
