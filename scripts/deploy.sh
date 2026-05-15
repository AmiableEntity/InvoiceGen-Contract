#!/bin/bash
# ─── Soroban Contract Deployment Script ──────────────────────────────────────
# Deploys the invoice contract to Stellar testnet.
# Prerequisites: stellar CLI installed (https://developers.stellar.org/docs/tools/stellar-cli)

set -e

NETWORK=${STELLAR_NETWORK:-testnet}
IDENTITY=${STELLAR_IDENTITY:-deployer}

echo "🔨 Building contract..."
cargo build --target wasm32-unknown-unknown --release

WASM_PATH="target/wasm32-unknown-unknown/release/stellar_invoice.wasm"

echo "📦 Optimizing WASM..."
stellar contract optimize --wasm "$WASM_PATH"

OPTIMIZED_WASM="target/wasm32-unknown-unknown/release/stellar_invoice.optimized.wasm"

echo "🚀 Deploying to $NETWORK..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$OPTIMIZED_WASM" \
  --source "$IDENTITY" \
  --network "$NETWORK")

echo "✅ Contract deployed!"
echo "📋 Contract ID: $CONTRACT_ID"

# Save contract ID to .env
echo "CONTRACT_ID=$CONTRACT_ID" >> .env

echo ""
echo "🔧 Initializing contract..."
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --source "$IDENTITY" \
  --network "$NETWORK" \
  -- initialize \
  --admin "$(stellar keys address $IDENTITY)"

echo "🎉 Contract initialized and ready!"
echo ""
echo "Add this to your frontend .env.local:"
echo "NEXT_PUBLIC_CONTRACT_ID=$CONTRACT_ID"
