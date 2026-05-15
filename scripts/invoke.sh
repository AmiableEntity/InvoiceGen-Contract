#!/bin/bash
# ─── Contract Invocation Examples ────────────────────────────────────────────
# Example commands for interacting with the deployed invoice contract.
# Load your CONTRACT_ID from .env first.

source .env

NETWORK=${STELLAR_NETWORK:-testnet}
IDENTITY=${STELLAR_IDENTITY:-deployer}

echo "Contract ID: $CONTRACT_ID"

# ─── Create an invoice ────────────────────────────────────────────────────────
echo "Creating invoice..."
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- create_invoice \
  --invoice_id "inv-test-001" \
  --freelancer "$(stellar keys address $IDENTITY)" \
  --client "GBBD47IF6LWK7P7MDEVSCWR7DPUWV3NY3DTQEVFL4NAT4AQH3ZLLFLA5" \
  --amount 10000000000 \
  --currency '{"tag":"Xlm"}' \
  --due_date 1800000000

# ─── Get invoice ──────────────────────────────────────────────────────────────
echo "Getting invoice..."
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- get_invoice \
  --invoice_id "inv-test-001"

# ─── Verify payment ───────────────────────────────────────────────────────────
echo "Verifying payment..."
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- verify_payment \
  --invoice_id "inv-test-001"
