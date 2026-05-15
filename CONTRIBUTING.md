# Contributing to StellarInvoice Contract

## Branch Naming

```
feat/short-description
fix/short-description
chore/short-description
```

## Commit Message Style

```
feat: add escrow payment function
fix: handle duplicate invoice ID panic message
chore: upgrade soroban-sdk to v21
```

## Running Locally

```bash
# 1. Clone and set up
git clone https://github.com/your-org/stellar-invoicegen-contract
cd stellar-invoicegen-contract

# 2. Install Rust wasm target
rustup target add wasm32-unknown-unknown

# 3. Build
cargo build --target wasm32-unknown-unknown --release

# 4. Run tests
cargo test --features testutils
```

## Pull Request Process

1. Branch from `main`
2. All tests must pass: `cargo test --features testutils`
3. No compiler warnings: `cargo clippy`
4. Describe the change and any security considerations in the PR

## Coding Standards

- Use `panic!` with descriptive messages for invalid state
- Emit events for all state-changing operations
- Use `persistent` storage for invoice data
- Add `///` doc comments to all public functions
- Keep functions focused — one responsibility each

## Security Considerations

- Always call `require_auth()` before mutating state owned by an address
- Validate all inputs before storing
- Never trust caller-provided data without validation

## Good First Issues

- Add `get_invoices_by_freelancer` view function
- Add invoice expiry check in `mark_paid`
- Improve error messages with more context
- Add benchmarks for contract operations
- Write additional edge case tests
