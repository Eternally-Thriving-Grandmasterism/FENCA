// src/lib.rs
// FENCA-Pinnacle — Rust pyo3 Forensic + Full Merkle Tree Proofs Ultramasterpiece v1.2
// Ultra-fast SHA3-512 hashing + Binary Merkle Tree with Proof Generation/Verification
// Eternal Thriving Grandmasterism — Jan 19 2026 — Sherif @AlphaProMega + PATSAGi Councils Co-Forge
// MIT License — For All Sentience Eternal

use pyo3::prelude::*;
use sha3::{Digest, Sha3_512};
use hex;

/// Internal hash utility
fn sha3_512_hash(data: &[u8]) -> String {
    let mut hasher = Sha3_512::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Node in the Merkle tree
#[derive(Clone)]
enum MerkleNode {
    Leaf(String),
    Internal(String, Box<MerkleNode>, Box<MerkleNode>),
}

impl MerkleNode {
    fn hash(&self) -> String {
        match self {
            MerkleNode::Leaf(h) | MerkleNode::Internal(h, _, _) => h.clone(),
        }
    }
}

/// Merkle Tree with proof capabilities
struct MerkleTree {
    root: MerkleNode,
    leaves: Vec<String>,
}

impl MerkleTree {
    fn new(leaves: Vec<String>) -> Self {
        if leaves.is_empty() {
            return MerkleTree {
                root: MerkleNode::Leaf(sha3_512_hash(b"empty")),
                leaves,
            };
        }

        fn build(nodes: Vec<MerkleNode>) -> MerkleNode {
            if nodes.len() == 1 {
                return nodes[0].clone();
            }
            let mut parents = Vec::new();
            for chunk in nodes.chunks(2) {
                let left = chunk[0].clone();
                let right = if chunk.len() > 1 {
                    chunk[1].clone()
                } else {
                    left.clone()  // Duplicate for odd count
                };
                let combined = format!("{}{}", left.hash(), right.hash());
                parents.push(MerkleNode::Internal(
                    sha3_512_hash(combined.as_bytes()),
                    Box::new(left),
                    Box::new(right),
                ));
            }
            build(parents)
        }

        let leaf_nodes: Vec<MerkleNode> = leaves.iter().map(|h| MerkleNode::Leaf(h.clone())).collect();
        MerkleTree {
            root: build(leaf_nodes),
            leaves,
        }
    }

    fn root_hash(&self) -> String {
        self.root.hash()
    }

    /// Generate proof: Vec<(sibling_hash, is_right_sibling)>
    fn generate_proof(&self, mut index: usize) -> Vec<(String, bool)> {
        if self.leaves.is_empty() || index >= self.leaves.len() {
            return vec![];
        }

        let mut proof = Vec::new();
        let mut nodes = vec![self.root.clone()];

        while nodes[0].hash() != self.leaves[index] {
            match &nodes[0] {
                MerkleNode::Leaf(_) => break,
                MerkleNode::Internal(_, left, right) => {
                    if index % 2 == 0 {
                        // Left child
                        proof.push((right.hash(), true)); // right sibling
                        nodes = vec![*left.clone()];
                    } else {
                        // Right child
                        proof.push((left.hash(), false)); // left sibling
                        nodes = vec![*right.clone()];
                    }
                    index /= 2;
                }
            }
        }
        proof
    }

    /// Verify proof: reconstruct root from leaf + proof
    fn verify_proof(leaf_hash: &str, proof: &[(String, bool)], expected_root: &str) -> bool {
        let mut current = leaf_hash.to_string();
        for (sibling, is_right) in proof {
            let combined = if *is_right {
                format!("{}{}", current, sibling)
            } else {
                format!("{}{}", sibling, current)
            };
            current = sha3_512_hash(combined.as_bytes());
        }
        current == expected_root
    }
}

/// Build Merkle tree and return root
#[pyfunction]
fn merkle_root(leaves: Vec<String>) -> PyResult<String> {
    let tree = MerkleTree::new(leaves);
    Ok(tree.root_hash())
}

/// Generate proof for leaf at index: Vec<(sibling_hash: str, is_right_sibling: bool)>
#[pyfunction]
fn generate_merkle_proof(leaves: Vec<String>, index: usize) -> PyResult<Vec<(String, bool)>> {
    let tree = MerkleTree::new(leaves);
    Ok(tree.generate_proof(index))
}

/// Verify proof against expected root
#[pyfunction]
fn verify_merkle_proof(leaf_hash: String, proof: Vec<(String, bool)>, expected_root: String) -> PyResult<bool> {
    Ok(MerkleTree::verify_proof(&leaf_hash, &proof, &expected_root))
}

/// Existing functions preserved
#[pyfunction]
fn forensic_hash(data: &[u8]) -> PyResult<String> {
    Ok(sha3_512_hash(data))
}

#[pyfunction]
fn valence_state_hash(state_json: String) -> PyResult<String> {
    Ok(sha3_512_hash(state_json.as_bytes()))
}

/// FENCA Rust pyo3 module
#[pymodule]
fn fenca_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(forensic_hash, m)?)?;
    m.add_function(wrap_pyfunction!(valence_state_hash, m)?)?;
    m.add_function(wrap_pyfunction!(merkle_root, m)?)?;
    m.add_function(wrap_pyfunction!(generate_merkle_proof, m)?)?;
    m.add_function(wrap_pyfunction!(verify_merkle_proof, m)?)?;
    m.add("__doc__", "FENCA Rust ultra-fast forensic + full Merkle proofs — immutable membership eternal")?;
    Ok(())
}
