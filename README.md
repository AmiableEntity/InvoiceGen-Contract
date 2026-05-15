# StellarInvoice — Smart Contract

Soroban smart contract (Rust) for on-chain invoice creation and payment tracking on the Stellar network.

## Features

- Create invoices on-chain with freelancer/client addresses
- Mark invoices as paid with a Stellar transaction hash
- Cancel pending invoices
- Verify payment status
- Event emission for all state changes
- Persistent storage across ledger archival

## Contract Functions

| Function | Auth Required | Description |
|---|---|---|
| `initialize(admin)` | Admin | One-time contract setup |
| `create_invoice(id, freelancer, client, amount, currency, due_date)` | Freelancer | Create a new invoice |
| `mark_paid(invoice_id, tx_hash)` | None | Mark invoice as paid |
| `cancel_invoice(invoice_id)` | Freelancer | Cancel a pending invoice |
| `get_invoice(invoice_id)` | None | Read invoice data |
| `invoice_exists(invoice_id)` | None | Check if invoice exists |
| `verify_payment(invoice_id)` | None | Returns true if paid |
| `get_payment_tx(invoice_id)` | None | Get payment tx hash |

## Invoice Lifecycle

```
create_invoice() → PENDING
                      ↓
                  mark_paid() → PAID
                      or
               cancel_invoice() → CANCELLED
```

## Setup

### Prerequisites

- [Rust](https://rustup.rs/) with `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/stellar-cli)

```bash
# Install Rust wasm target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt
```

### Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Test

```bash
cargo test --features testutils
```

### Deploy to Testnet

1. Create a Stellar identity:
```bash
stellar keys generate deployer --network testnet
stellar keys fund deployer --network testnet
```

2. Copy and configure environment:
```bash
cp .env.example .env
```

3. Run deploy script:
```bash
chmod +x scripts/deploy.sh
./scripts/deploy.sh
```

4. Copy the `CONTRACT_ID` output to your frontend and backend `.env` files.

### Invoke Contract

See `scripts/invoke.sh` for example invocations.

## Environment Variables

| Variable | Description |
|---|---|
| `STELLAR_NETWORK` | `testnet` or `mainnet` |
| `STELLAR_IDENTITY` | Stellar CLI key name |
| `CONTRACT_ID` | Deployed contract address |

## Deployment Notes

- Always test on testnet before mainnet
- Use `stellar contract optimize` to reduce WASM size before deployment
- Contract storage uses `persistent` — entries survive ledger archival
- Events are emitted for `created`, `paid`, and `cancelled` state changes

## Future Improvements

- Escrow payment flow (hold funds in contract until work approved)
- Dispute resolution mechanism
- Multi-signature approval for large invoices
- Automatic overdue detection via ledger timestamp
