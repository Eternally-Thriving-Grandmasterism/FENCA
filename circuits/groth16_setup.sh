#!/bin/bash
# groth16_setup.sh — FENCA Pinnacle Full Groth16 Trusted Setup for MerkleProof(20) Circuit
# Powrush Ultramasterpiece — Complete SNARK Pipeline: Compile → Setup → Contribute → Beacon → Verify → Export
# Eternal Thriving Grandmasterism — Jan 19 2026 — Sherif @AlphaProMega + PATSAGi Councils Co-Forge
# MIT License — For All Sentience Eternal

# Prerequisites:
# npm install -g snarkjs circom
# Download powersOfTau28_hez_final_16.ptau from Hermez trusted setup

set -e  # Exit on any error

CIRCUIT_NAME="merkle_proof"
BUILD_DIR="build"
PTAU="powersOfTau28_hez_final_16.ptau"

echo "=== FENCA Groth16 Setup Initiated — Private Rapture Proofs Eternal ==="

# Step 1: Compile circuit
echo "Compiling $CIRCUIT_NAME.circom..."
circom ${CIRCUIT_NAME}.circom --r1cs --wasm --sym -o $BUILD_DIR

# Step 2: Phase 1 — Start new zkey (requires ptau file)
echo "Starting new Groth16 zkey from trusted ptau..."
snarkjs groth16 setup \( BUILD_DIR/ \){CIRCUIT_NAME}.r1cs $PTAU \( BUILD_DIR/ \){CIRCUIT_NAME}_0000.zkey

# Step 3: Phase 2 — Contribute (add entropy)
echo "Phase 2 contribution — add your entropy..."
snarkjs zkey contribute \( BUILD_DIR/ \){CIRCUIT_NAME}_0000.zkey \( BUILD_DIR/ \){CIRCUIT_NAME}_0001.zkey \
    --name="1st Contributor - FENCA Pinnacle" -v -e="FENCA entropy beacon 2026"

# Optional additional contributions (repeat for multi-party)
# snarkjs zkey contribute \( BUILD_DIR/ \){CIRCUIT_NAME}_0001.zkey \( BUILD_DIR/ \){CIRCUIT_NAME}_0002.zkey \
#     --name="2nd Contributor" -v -e="additional entropy"

# Step 4: Phase 2 beacon (finalization)
echo "Applying phase 2 beacon for final zkey..."
snarkjs zkey beacon \( BUILD_DIR/ \){CIRCUIT_NAME}_0001.zkey \( BUILD_DIR/ \){CIRCUIT_NAME}_final.zkey \
    0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f 10 -n="Final Beacon - FENCA Eternal"

# Step 5: Verify final zkey
echo "Verifying final zkey..."
snarkjs zkey verify \( BUILD_DIR/ \){CIRCUIT_NAME}.r1cs $PTAU \( BUILD_DIR/ \){CIRCUIT_NAME}_final.zkey

# Step 6: Export verification key
echo "Exporting verification key..."
snarkjs zkey export verificationkey \( BUILD_DIR/ \){CIRCUIT_NAME}_final.zkey $BUILD_DIR/verification_key.json

echo "=== Groth16 Setup Complete — Private Merkle Proofs Ready Eternal ==="
echo "Next: Use snarkjs groth16 fullprove for proof generation"
echo "Verification key: $BUILD_DIR/verification_key.json"
echo "Final proving key: \( BUILD_DIR/ \){CIRCUIT_NAME}_final.zkey"
