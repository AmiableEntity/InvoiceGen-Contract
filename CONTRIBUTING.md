# Contributing to StellarInvoice Contract

Thanks for your interest! This is a Rust/Soroban project — if you're new to either, don't worry. Many issues don't require deep Rust knowledge, and we're happy to help you get started.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Ways to Contribute](#ways-to-contribute)
- [Good First Issues](#good-first-issues)
- [Local Setup](#local-setup)
- [Branch Naming](#branch-naming)
- [Commit Messages](#commit-messages)
- [Pull Request Process](#pull-request-process)
- [Coding Standards](#coding-standards)
- [Security Considerations](#security-considerations)
- [Reporting Bugs](#reporting-bugs)
- [Suggesting Features](#suggesting-features)

---

## Code of Conduct

Be respectful and patient. Smart contract development has a learning curve — we welcome questions and support contributors at all levels.

---

## Ways to Contribute

- **Write tests** — we always need more edge case coverage
- **Improve error messages** — make panics more descriptive
- **Add a view function** — read-only functions are low risk and great for beginners
- **Fix a bug** — pick an open issue
- **Improve docs** — clarify deployment steps, add inline code comments
- **Review PRs** — even non-Rust developers can review logic and docs

---

## Good First Issues

These are well-scoped and approachable:

- [ ] Add `get_invoice_status(invoice_id)` — returns just the status enum (simpler than `get_invoice`)
- [ ] Add overdue detection — `is_overdue(invoice_id)` returns true if `due_date < ledger.timestamp()` and status is Pending
- [ ] Improve panic messages — add invoice ID to "Invoice not found" panics
- [ ] Add a test for `cancel_invoice` on an already-cancelled invoice
- [ ] Add a test verifying events are emitted on `create_invoice`
- [ ] Add `get_payment_tx` test for unpaid invoice (should return `None`)
- [ ] Document all function parameters with inline `///` comments
- [ ] Add `scripts/test.sh` — a convenience script that runs `cargo test --features testutils`

Look for issues tagged [`good first issue`](https://github.com/AmiableEntity/InvoiceGen-Contract/issues?q=label%3A%22good+first+issue%22) on GitHub.

---

## Local Setup

```bash
# 1. Install Rust (if you haven't)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 2. Add the wasm target
rustup target add wasm32-unknown-unknown

# 3. Install Stellar CLI
cargo install --locked stellar-cli --features opt

# 4. Fork the repo, then clone your fork
git clone https://github.com/YOUR_USERNAME/InvoiceGen-Contract.git
cd InvoiceGen-Contract

# 5. Build
cargo build --target wasm32-unknown-unknown --release

# 6. Run tests
cargo test --features testutils
```

All tests should pass before you start making changes.

---

## Branch Naming

```
feat/short-description        New function or feature
fix/short-description         Bug fix
test/short-description        Tests only
docs/short-description        Documentation only
chore/short-description       Tooling, deps, config
```

Examples:
```
feat/is-overdue-function
fix/cancel-already-cancelled-panic
test/event-emission-coverage
```

---

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add is_overdue view function
fix: include invoice_id in not found panic message
test: add edge case tests for mark_paid
docs: add inline comments to create_invoice params
chore: upgrade soroban-sdk to v21
```

Rules:
- Present tense ("add" not "added")
- First line under 72 characters
- Add a body for non-obvious changes

---

## Pull Request Process

1. **Fork** and branch from `main`
2. **Make your changes** — keep them focused
3. **Verify before opening the PR:**
   ```bash
   cargo build --target wasm32-unknown-unknown --release   # must compile
   cargo test --features testutils                          # all tests must pass
   cargo clippy -- -D warnings                             # no clippy warnings
   ```
4. **Open the PR** with:
   - A clear title following commit message style
   - What changed and why
   - Any security implications (especially for write functions)
   - `Closes #123` if it resolves an issue
5. **Respond to feedback** — contract changes get careful review, especially anything touching auth or state mutation

---

## Coding Standards

- **Auth first** — call `address.require_auth()` at the top of any function that mutates state owned by an address, before any other logic
- **Validate inputs** — check amounts, statuses, and existence before writing to storage
- **Descriptive panics** — `panic!("Invoice not found: {}", id)` is better than `panic!("not found")`
- **Emit events** — every state-changing function should emit an event for off-chain indexers
- **Persistent storage** — use `env.storage().persistent()` for invoice data so it survives ledger archival
- **Doc comments** — all public functions need `///` comments explaining purpose, parameters, and panic conditions
- **No unsafe** — do not use `unsafe` blocks

---

## Security Considerations

Smart contracts are immutable once deployed. Security issues are especially important here:

- **Always call `require_auth()`** before mutating state that belongs to an address
- **Never trust caller-provided data** without validation
- **Check state transitions** — a paid invoice should never be paid again, a cancelled invoice should never be paid
- **Avoid unbounded loops** — Soroban has instruction limits
- If you find a security vulnerability, please **open a private security advisory** on GitHub rather than a public issue

---

## Reporting Bugs

Open an issue with:

1. The function that triggered the bug
2. The input values used
3. Expected behavior vs actual behavior
4. Whether it's a logic bug or a panic

---

## Suggesting Features

Open an issue describing:

1. The use case
2. The proposed function signature
3. Auth requirements (who can call it?)
4. Any state changes and their implications

Discuss before building — contract design decisions have long-term consequences.

---

Thanks for contributing. Careful, well-tested contract code is what makes this system trustworthy.
