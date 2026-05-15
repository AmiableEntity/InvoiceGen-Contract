//! Integration tests for the Stellar Invoice contract.
//! Run with: cargo test --features testutils

#[cfg(test)]
mod tests {
    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        Address, Bytes, Env, String,
    };
    use stellar_invoice::{Currency, InvoiceContract, InvoiceContractClient, InvoiceStatus};

    /// Helper: create a test environment with the contract deployed
    fn setup() -> (Env, InvoiceContractClient<'static>, Address, Address, Address) {
        let env = Env::default();
        env.mock_all_auths(); // Auto-approve all auth checks in tests

        let contract_id = env.register_contract(None, InvoiceContract);
        let client = InvoiceContractClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        let freelancer = Address::generate(&env);
        let client_addr = Address::generate(&env);

        // Initialize contract
        client.initialize(&admin);

        (env, client, admin, freelancer, client_addr)
    }

    #[test]
    fn test_create_invoice() {
        let (env, client, _admin, freelancer, client_addr) = setup();

        let invoice_id = String::from_str(&env, "inv-001");
        let amount: i128 = 1_000_0000000; // 1000 XLM in stroops
        let due_date: u64 = 1_800_000_000;

        let invoice = client.create_invoice(
            &invoice_id,
            &freelancer,
            &client_addr,
            &amount,
            &Currency::Xlm,
            &due_date,
        );

        assert_eq!(invoice.id, invoice_id);
        assert_eq!(invoice.freelancer, freelancer);
        assert_eq!(invoice.amount, amount);
        assert_eq!(invoice.status, InvoiceStatus::Pending);
        assert!(invoice.payment_tx_hash.is_none());
    }

    #[test]
    fn test_mark_paid() {
        let (env, client, _admin, freelancer, client_addr) = setup();

        let invoice_id = String::from_str(&env, "inv-002");
        let tx_hash = Bytes::from_slice(&env, b"abc123txhash");

        client.create_invoice(
            &invoice_id,
            &freelancer,
            &client_addr,
            &500_0000000_i128,
            &Currency::Usdc,
            &1_800_000_000_u64,
        );

        let paid_invoice = client.mark_paid(&invoice_id, &tx_hash);

        assert_eq!(paid_invoice.status, InvoiceStatus::Paid);
        assert_eq!(paid_invoice.payment_tx_hash, Some(tx_hash));
    }

    #[test]
    fn test_verify_payment_returns_true_when_paid() {
        let (env, client, _admin, freelancer, client_addr) = setup();

        let invoice_id = String::from_str(&env, "inv-003");
        let tx_hash = Bytes::from_slice(&env, b"txhash456");

        client.create_invoice(
            &invoice_id,
            &freelancer,
            &client_addr,
            &100_0000000_i128,
            &Currency::Xlm,
            &1_800_000_000_u64,
        );

        // Not paid yet
        assert!(!client.verify_payment(&invoice_id));

        client.mark_paid(&invoice_id, &tx_hash);

        // Now paid
        assert!(client.verify_payment(&invoice_id));
    }

    #[test]
    fn test_cancel_invoice() {
        let (env, client, _admin, freelancer, client_addr) = setup();

        let invoice_id = String::from_str(&env, "inv-004");

        client.create_invoice(
            &invoice_id,
            &freelancer,
            &client_addr,
            &200_0000000_i128,
            &Currency::Usdc,
            &1_800_000_000_u64,
        );

        let cancelled = client.cancel_invoice(&invoice_id);
        assert_eq!(cancelled.status, InvoiceStatus::Cancelled);
    }

    #[test]
    #[should_panic(expected = "Invoice is already paid")]
    fn test_cannot_pay_twice() {
        let (env, client, _admin, freelancer, client_addr) = setup();

        let invoice_id = String::from_str(&env, "inv-005");
        let tx_hash = Bytes::from_slice(&env, b"txhash789");

        client.create_invoice(
            &invoice_id,
            &freelancer,
            &client_addr,
            &300_0000000_i128,
            &Currency::Xlm,
            &1_800_000_000_u64,
        );

        client.mark_paid(&invoice_id, &tx_hash.clone());
        // This should panic
        client.mark_paid(&invoice_id, &tx_hash);
    }

    #[test]
    #[should_panic(expected = "Invoice amount must be positive")]
    fn test_zero_amount_rejected() {
        let (env, client, _admin, freelancer, client_addr) = setup();

        let invoice_id = String::from_str(&env, "inv-006");

        client.create_invoice(
            &invoice_id,
            &freelancer,
            &client_addr,
            &0_i128, // invalid
            &Currency::Xlm,
            &1_800_000_000_u64,
        );
    }

    #[test]
    fn test_invoice_exists() {
        let (env, client, _admin, freelancer, client_addr) = setup();

        let invoice_id = String::from_str(&env, "inv-007");

        assert!(!client.invoice_exists(&invoice_id));

        client.create_invoice(
            &invoice_id,
            &freelancer,
            &client_addr,
            &100_0000000_i128,
            &Currency::Xlm,
            &1_800_000_000_u64,
        );

        assert!(client.invoice_exists(&invoice_id));
    }
}
