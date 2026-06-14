//! # Stellar Invoice Contract
//!
//! A Soroban smart contract for creating and tracking invoices on the Stellar network.
//!
//! ## Lifecycle
//! 1. Freelancer calls `create_invoice` → invoice stored on-chain with PENDING status
//! 2. Client pays via a standard Stellar payment transaction
//! 3. Anyone calls `mark_paid` with the tx hash → invoice status updated to PAID
//! 4. `verify_payment` can be called to check payment status at any time

#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Bytes, Env, String,
    events,
};

// ─── Data Types ───────────────────────────────────────────────────────────────

/// Invoice payment status
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum InvoiceStatus {
    Pending,
    Paid,
    Cancelled,
}

/// Currency type for the invoice
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Currency {
    Xlm,
    Usdc,
}

/// Core invoice data stored on-chain
#[contracttype]
#[derive(Clone, Debug)]
pub struct Invoice {
    /// Unique invoice ID (matches backend UUID)
    pub id: String,
    /// Freelancer's Stellar address (payment recipient)
    pub freelancer: Address,
    /// Client's Stellar address (optional — for reference)
    pub client: Address,
    /// Invoice amount in stroops (XLM) or smallest unit (USDC)
    /// Stored as i128 to match Stellar asset amounts
    pub amount: i128,
    /// Payment currency
    pub currency: Currency,
    /// Unix timestamp of invoice creation
    pub created_at: u64,
    /// Unix timestamp of due date
    pub due_date: u64,
    /// Current payment status
    pub status: InvoiceStatus,
    /// Stellar transaction hash of the payment (set when paid)
    pub payment_tx_hash: Option<Bytes>,
}

/// Storage key types for contract state
#[contracttype]
pub enum DataKey {
    /// Invoice storage: DataKey::Invoice(invoice_id) → Invoice
    Invoice(String),
    /// Admin address (contract deployer)
    Admin,
}

// ─── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct InvoiceContract;

#[contractimpl]
impl InvoiceContract {
    // ─── Initialization ───────────────────────────────────────────────────────

    /// Initialize the contract with an admin address.
    /// Must be called once after deployment.
    pub fn initialize(env: Env, admin: Address) {
        // Prevent re-initialization
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);

        // Emit initialization event
        env.events().publish(
            (symbol_short!("init"),),
            admin,
        );
    }

    // ─── Invoice Creation ─────────────────────────────────────────────────────

    /// Create a new invoice on-chain.
    ///
    /// # Arguments
    /// * `invoice_id` - Unique ID matching the backend database record
    /// * `freelancer` - Address that will receive payment
    /// * `client` - Address of the paying client
    /// * `amount` - Payment amount (in stroops for XLM, or 7-decimal units for USDC)
    /// * `currency` - XLM or USDC
    /// * `due_date` - Unix timestamp of the due date
    ///
    /// # Panics
    /// - If an invoice with this ID already exists
    /// - If amount is zero or negative
    pub fn create_invoice(
        env: Env,
        invoice_id: String,
        freelancer: Address,
        client: Address,
        amount: i128,
        currency: Currency,
        due_date: u64,
    ) -> Invoice {
        // Only the freelancer can create their own invoice
        freelancer.require_auth();

        // Validate amount
        if amount <= 0 {
            panic!("Invoice amount must be positive");
        }

        // Ensure invoice ID is unique
        let key = DataKey::Invoice(invoice_id.clone());
        if env.storage().persistent().has(&key) {
            panic!("Invoice with this ID already exists");
        }

        let invoice = Invoice {
            id: invoice_id.clone(),
            freelancer: freelancer.clone(),
            client: client.clone(),
            amount,
            currency: currency.clone(),
            created_at: env.ledger().timestamp(),
            due_date,
            status: InvoiceStatus::Pending,
            payment_tx_hash: None,
        };

        // Store invoice with persistent storage (survives ledger archival)
        env.storage().persistent().set(&key, &invoice);

        // Emit creation event for indexers
        env.events().publish(
            (symbol_short!("created"), invoice_id),
            (freelancer, client, amount),
        );

        invoice
    }

    // ─── Mark as Paid ─────────────────────────────────────────────────────────

    /// Mark an invoice as paid after verifying the Stellar transaction.
    ///
    /// # Arguments
    /// * `invoice_id` - The invoice to mark as paid
    /// * `tx_hash` - The Stellar transaction hash of the payment
    ///
    /// # Panics
    /// - If invoice not found
    /// - If invoice is already paid or cancelled
    pub fn mark_paid(env: Env, invoice_id: String, tx_hash: Bytes) -> Invoice {
        let key = DataKey::Invoice(invoice_id.clone());

        let mut invoice: Invoice = env
            .storage()
            .persistent()
            .get(&key)
            .expect("Invoice not found");

        // Validate current status
        match invoice.status {
            InvoiceStatus::Paid => panic!("Invoice is already paid"),
            InvoiceStatus::Cancelled => panic!("Cannot pay a cancelled invoice"),
            InvoiceStatus::Pending => {}
        }

        // Update invoice state
        invoice.status = InvoiceStatus::Paid;
        invoice.payment_tx_hash = Some(tx_hash.clone());

        // Persist updated invoice
        env.storage().persistent().set(&key, &invoice);

        // Emit payment event
        env.events().publish(
            (symbol_short!("paid"), invoice_id),
            (invoice.freelancer.clone(), invoice.amount, tx_hash),
        );

        invoice
    }

    // ─── Cancel Invoice ───────────────────────────────────────────────────────

    /// Cancel a pending invoice. Only the freelancer can cancel.
    ///
    /// # Panics
    /// - If invoice not found
    /// - If invoice is already paid
    pub fn cancel_invoice(env: Env, invoice_id: String) -> Invoice {
        let key = DataKey::Invoice(invoice_id.clone());

        let mut invoice: Invoice = env
            .storage()
            .persistent()
            .get(&key)
            .expect("Invoice not found");

        // Only the freelancer can cancel
        invoice.freelancer.require_auth();

        if invoice.status == InvoiceStatus::Paid {
            panic!("Cannot cancel a paid invoice");
        }

        invoice.status = InvoiceStatus::Cancelled;
        env.storage().persistent().set(&key, &invoice);

        env.events().publish(
            (symbol_short!("cancelled"), invoice_id),
            invoice.freelancer.clone(),
        );

        invoice
    }

    // ─── Read Functions ───────────────────────────────────────────────────────

    /// Get an invoice by ID.
    ///
    /// # Panics
    /// - If invoice not found
    pub fn get_invoice(env: Env, invoice_id: String) -> Invoice {
        let key = DataKey::Invoice(invoice_id);
        env.storage()
            .persistent()
            .get(&key)
            .expect("Invoice not found")
    }

    /// Check if an invoice exists.
    pub fn invoice_exists(env: Env, invoice_id: String) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Invoice(invoice_id))
    }

    /// Verify payment status of an invoice.
    /// Returns true if the invoice is in PAID status.
    pub fn verify_payment(env: Env, invoice_id: String) -> bool {
        let key = DataKey::Invoice(invoice_id);
        if let Some(invoice) = env.storage().persistent().get::<DataKey, Invoice>(&key) {
            invoice.status == InvoiceStatus::Paid
        } else {
            false
        }
    }

    /// Check if a pending invoice is overdue.
    /// Returns true if the invoice is still Pending and the due date has passed.
    ///
    /// # Arguments
    /// * `invoice_id` - The invoice to check
    ///
    /// # Panics
    /// - If invoice not found
    pub fn is_overdue(env: Env, invoice_id: String) -> bool {
        let key = DataKey::Invoice(invoice_id.clone());
        let invoice: Invoice = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("Invoice not found: {}", invoice_id));
        invoice.status == InvoiceStatus::Pending && env.ledger().timestamp() > invoice.due_date
    }

    /// Get just the status of an invoice without fetching the full struct.
    ///
    /// # Arguments
    /// * `invoice_id` - The invoice to query
    ///
    /// # Panics
    /// - If invoice not found
    pub fn get_invoice_status(env: Env, invoice_id: String) -> InvoiceStatus {
        let key = DataKey::Invoice(invoice_id.clone());
        let invoice: Invoice = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| panic!("Invoice not found: {}", invoice_id));
        invoice.status
    }

    /// Get the payment transaction hash for a paid invoice.
    /// Returns None if not paid yet.
    pub fn get_payment_tx(env: Env, invoice_id: String) -> Option<Bytes> {
        let key = DataKey::Invoice(invoice_id);
        let invoice: Invoice = env
            .storage()
            .persistent()
            .get(&key)
            .expect("Invoice not found");
        invoice.payment_tx_hash
    }
}
