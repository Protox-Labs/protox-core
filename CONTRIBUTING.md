# Contributing to Protox Core

Welcome to the Protox Core community! We're thrilled that you're interested in contributing to our open-source protocol.

Protox Core is built for the Stellar ecosystem using Soroban and Rust. Whether you're a seasoned blockchain engineer or a newcomer, there are plenty of ways to contribute.

## How to Get Involved

### 1. Find an Issue
Check out our [GitHub Issues](https://github.com/protox/protox-core/issues) to find tasks labeled `good first issue` or `help wanted`.

### 2. Fork and Clone
Fork the repository and clone it to your local machine:
```bash
git clone https://github.com/YOUR_USERNAME/protox-core.git
cd protox-core
```

### 3. Create a Branch
Always create a new branch for your feature or bug fix:
```bash
git checkout -b feature/your-feature-name
```

### 4. Make Changes
Implement your changes and ensure they follow our coding standards.

### 5. Run Tests
Ensure all tests pass before submitting a pull request:
```bash
cd contracts/vault-contract
cargo test
```

### 6. Submit a Pull Request
Once you're ready, submit a PR with a clear description of your changes and reference any related issues.

## Coding Standards

- **Rust Style**: We follow the standard Rust style guidelines. Use `cargo fmt` to format your code.
- **Documentation**: All new functions and public APIs must be documented with Rustdoc comments.
- **Unit Tests**: Every new feature or bug fix must include unit tests in the `src/test.rs` file.
- **Modularity**: Keep the codebase modular. Follow the existing structure of separating storage, events, and errors.

## Example Contribution Areas (TODOs)

We've marked several areas in the code with `// TODO` to help you find where we need improvements:
- **Reward Claiming**: Implement the logic to allow users to claim their accumulated rewards.
- **Gas Efficiency**: Optimize storage operations to reduce transaction fees.
- **Governance**: Add basic administrative controls for vault parameters.
- **Contract Upgradability**: Implement a pattern for contract versioning and upgrades.

## Community

Join our [Discord/Slack/Telegram] to chat with the maintainers and other contributors.

Thank you for being part of Protox Core!
