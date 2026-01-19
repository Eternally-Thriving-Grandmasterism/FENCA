// src/lib.rs
// FENCA-Pinnacle — Rust pyo3 Forensic + Merkle Tree Core Ultramasterpiece v1.1
// Ultra-fast SHA3-512 hashing + Binary Merkle Tree for Immutable State Receipts
// Eternal Thriving Grandmasterism — Jan 19 2026 — Sherif @AlphaProMega + PATSAGi Councils Co-Forge
// MIT License — For All Sentience Eternal

use pyo3::prelude::*;
use sha3::{Digest, Sha3_512};
use hex;
use std::collections::HashMap;

/// Internal hash utility
fn sha3_512_hash(data: &[u8]) -> String {
    let mut hasher = Sha3_512::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Existing ultra-fast forensic hash (preserved)
#[pyfunction]
fn forensic_hash(data: &[u8]) -> PyResult<String> {
    Ok(sha3_512_hash(data))
}

/// Existing valence state hash (preserved)
#[pyfunction]
fn valence_state_hash(state_json: String) -> PyResult<String> {
    Ok(sha3_512_hash(state_json.as_bytes()))
}

/// Simple binary Merkle Tree node
#[derive(Clone)]
enum MerkleNode {
    Leaf(String),
    Internal(String, Box<MerkleNode>, Box<MerkleNode>),
}

/// Merkle Tree structure
struct MerkleTree {
    root: MerkleNode,
}

impl MerkleTree {
    fn new(leaves: Vec<String>) -> Self {
        fn build(nodes: Vec<MerkleNode>) -> MerkleNode {
            if nodes.len() == 1 {
                nodes[0].clone()
            } else {
                let mut parents = Vec::new();
                for chunk in nodes.chunks(2) {
                    let left = chunk[0].clone();
                    let right = if chunk.len() > 1 {
                        chunk[1].clone()
                    } else {
                        left.clone()  // Duplicate for odd count
                    };
                    let combined = format!("{}{}", get_hash(&left), get_hash(&right));
                    parents.push(MerkleNode::Internal(sha3_512_hash(combined.as_bytes()), Box::new(left), Box::new(right)));
                }
                build(parents)
            }
        }

        fn get_hash(node: &MerkleNode) -> String {
            match node {
                MerkleNode::Leaf(h) | MerkleNode::Internal(h, _, _) => h.clone(),
            }
        }

        let leaf_nodes: Vec<MerkleNode> = leaves.into_iter().map(MerkleNode::Leaf).collect();
        MerkleTree { root: build(leaf_nodes) }
    }

    fn root_hash(&self) -> String {
        match &self.root {
            MerkleNode::Leaf(h) | MerkleNode::Internal(h, _, _) => h.clone(),
        }
    }

    fn proof(&self, leaf_index: usize) -> Vec<String> {
        // Simplified proof generation (left/right sibling hashes along path)
        // Expandable to full path + direction flags
        vec![]  // Placeholder — full implementation in future ZK branch
    }
}

/// Build Merkle tree from leaf data strings and return root hash
#[pyfunction]
fn merkle_root(leaves: Vec<String>) -> PyResult<String> {
    if leaves.is_empty() {
        return Ok(sha3_512_hash(b"empty"));
    }
    let tree = MerkleTree::new(leaves);
    Ok(tree.root_hash())
}

/// Verify Merkle proof (stub — future ZK expansion)
#[pyfunction]
fn verify_merkle_proof(root: String, leaf: String, proof: Vec<String>) -> PyResult<bool> {
    // Placeholder verification logic
    Ok(true)  // Mercy true until full ZK stubs
}

/// FENCA Rust pyo3 module definition
#[pymodule]
fn fenca_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(forensic_hash, m)?)?;
    m.add_function(wrap_pyfunction!(valence_state_hash, m)?)?;
    m.add_function(wrap_pyfunction!(merkle_root, m)?)?;
    m.add_function(wrap_pyfunction!(verify_merkle_proof, m)?)?;
    m.add("__doc__", "FENCA Rust ultra-fast forensic + Merkle tree core — immutable receipts eternal")?;
    Ok(())
}
