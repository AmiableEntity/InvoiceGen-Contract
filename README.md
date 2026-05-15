# StellarInvoice — Smart Contract

> Soroban smart contract for on-chain invoice creation and payment tracking on the Stellar network. Written in Rust.

![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)
![Soroban](https://img.shields.io/badge/Soroban-SDK%2020-7B2FBE)
![Stellar](https://img.shields.io/badge/Stellar-Testnet-3E1BDB?logo=stellar)
![License](https://img.shields.io/badge/license-MIT-green)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)

---

## What is this?

This Soroban contract stores invoice state on the Stellar blockchain. It provides:

- On-chain invoice creation with freelancer/client addresses
- Payment tracking with Stellar transaction hash recording
- Immutable audit trail of all invoice state changes
- Events emitted on every state transition (for indexers)

The contract is an optional layer — the [backend](https://github.com/AmiableEntity/InvoiceGen-backend) can verify payments directly via Horizon without it. The contract adds full on-chain transparency.

---

## Invoice Lifecycle

```
create_invoice()
       │
       ▼
   [ PENDING ]
       │
       ├──── mark_paid(tx_hash) ──────► [ PAID ]
       │
       └──── cancel_invoice() ─────────► [ CANCELLED ]
```

---

## Contract Functions

### Write Functions (change state)

| Function | Auth | Description |
|---|---|---|
| `initialize(admin)` | Admin | One-time setup after deployment |
| `create_invoice(id, freelancer, client, amount, currency, due_date)` | Freelancer | Create a new invoice on-chain |
| `mark_paid(invoice_id, tx_hash)` | None | Record payment and mark invoice paid |
| `cancel_invoice(invoice_id)` | Freelancer | Cancel a pending invoice |

### Read Functions (no state change, free to call)

| Function | Description |
|---|---|
| `get_invoice(invoice_id)` | Returns full invoice struct |
| `invoice_exists(invoice_id)` | Returns bool |
| `verify_payment(invoice_id)` | Returns true if status is PAID |
| `get_payment_tx(invoice_id)` | Returns payment tx hash if paid |

---

## Data Types

```rust
struct Invoice {
    id: String,               // Matches backend UUID
    freelancer: Address,      // Payment recipient
    client: Address,          // Paying party
    amount: i128,             // In stroops (XLM) or 7-decimal units (USDC)
    currency: Currency,       // Xlm | Usdc
    created_at: u64,          // Ledger timestamp
    due_date: u64,            // Unix timestamp
    status: InvoiceStatus,    // Pending | Paid | Cancelled
    payment_tx_hash: Option<Bytes>,
}
```

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/stellar-cli)

```bash
# Install Rust wasm target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install --locked stellar-cli --features opt
```

### 1. Clone and build

```bash
git clone https://github.com/AmiableEntity/InvoiceGen-Contract.git
cd InvoiceGen-Contract

cargo build --target wasm32-unknown-unknown --release
```

### 2. Run tests

```bash
cargo test --features testutils
```

### 3. Deploy to Testnet

```bash
# Create and fund a testnet identity
stellar keys generate deployer --network testnet
stellar keys fund deployer --network testnet

# Copy environment config
cp .env.example .env

# Run the deploy script
chmod +x scripts/deploy.sh
./scripts/deploy.sh
```

The script will output your `CONTRACT_ID`. Add it to your frontend and backend `.env` files.

---

## Environment Variables

| Variable | Description |
|---|---|
| `STELLAR_NETWORK` | `testnet` or `mainnet` |
| `STELLAR_IDENTITY` | Stellar CLI key name (e.g. `deployer`) |
| `CONTRACT_ID` | Set automatically by `deploy.sh` |
| `STELLAR_HORIZON_URL` | Horizon server URL |

---

## Invoking the Contract

See `scripts/invoke.sh` for ready-to-run examples. Quick reference:

```bash
# Create an invoice
stellar contract invoke \
  --id $CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- create_invoice \
  --invoice_id "inv-001" \
  --freelancer "G..." \
  --client "G..." \
  --amount 10000000000 \
  --currency '{"tag":"Usdc"}' \
  --due_date 1800000000

# Check payment status
stellar contract invoke \
  --id $CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- verify_payment \
  --invoice_id "inv-001"
```

---

## Deployment Notes

- Always test on testnet before mainnet
- Run `stellar contract optimize` to reduce WASM size before deploying
- Contract uses `persistent` storage — data survives ledger archival
- Events are emitted for `created`, `paid`, and `cancelled` — useful for off-chain indexers
- `initialize()` can only be called once — it will panic on re-initialization

---

## Related Repos

| Repo | Description |
|---|---|
| [InvoiceGen-frontend](https://github.com/AmiableEntity/InvoiceGen-frontend) | Next.js frontend |
| [InvoiceGen-backend](https://github.com/AmiableEntity/InvoiceGen-backend) | Express + PostgreSQL API |

---

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md). Rust experience helpful but not required for many issues.

---

## Future Improvements

- Escrow payment flow (hold funds in contract until work is approved)
- Dispute resolution mechanism
- Automatic overdue detection via ledger timestamp
- Multi-signature approval for large invoices
- Batch invoice creation

---

## License

MIT — see [LICENSE](./LICENSE)
