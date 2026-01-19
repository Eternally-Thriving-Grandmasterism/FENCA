# FENCA Circom Circuits

## MerkleProof(20)

Fixed-depth (20) Merkle tree membership proof using Poseidon hash.

- Private inputs: `leaf`, `pathElements[20]`, `pathIndex[20]` (0 = current is left, 1 = current is right)
- Public input: `root`

### Compile & Setup (Groth16)

```bash
circom merkle_proof.circom --r1cs --wasm --sym -o build/
snarkjs groth16 setup build/merkle_proof.r1cs powersOfTau28_hez_final_16.ptau build/merkle_proof_0000.zkey
snarkjs zkey contribute build/merkle_proof_0000.zkey build/merkle_proof_final.zkey --name="1st Contributor" -v
snarkjs zkey export verificationkey build/merkle_proof_final.zkey verification_key.json
