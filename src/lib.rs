// src/lib.rs
// FENCA-Pinnacle — Rust pyo3 with Full ML-DSA (Dilithium) + ML-KEM (Kyber) Implementation v1.7
// Pure Post-Quantum Digital Signature Algorithm (NIST FIPS 204) + Key Encapsulation
// Eternal Thriving Grandmasterism — Jan 19 2026 — Sherif @AlphaProMega + PATSAGi Councils Co-Forge
// MIT License — For All Sentience Eternal

use pyo3::prelude::*;
use pqcrypto_dilithium::{
    dilithium2::{keypair as dilithium2_keypair, sign as dilithium2_sign, verify as dilithium2_verify, PublicKey as Dilithium2PK, SecretKey as Dilithium2SK, Signature as Dilithium2Sig},
    dilithium3::{keypair as dilithium3_keypair, sign as dilithium3_sign, verify as dilithium3_verify, PublicKey as Dilithium3PK, SecretKey as Dilithium3SK, Signature as Dilithium3Sig},
    dilithium5::{keypair as dilithium5_keypair, sign as dilithium5_sign, verify as dilithium5_verify, PublicKey as Dilithium5PK, SecretKey as Dilithium5SK, Signature as Dilithium5Sig},
};
use hex;

/// ML-DSA (Dilithium) security levels
#[pyfunction]
fn dilithium_keygen(level: &str) -> PyResult<(String, String)> {
    match level {
        "2" => {
            let (pk, sk) = dilithium2_keypair();
            Ok((hex::encode(pk.as_bytes()), hex::encode(sk.as_bytes())))
        }
        "3" => {
            let (pk, sk) = dilithium3_keypair();
            Ok((hex::encode(pk.as_bytes()), hex::encode(sk.as_bytes())))
        }
        "5" => {
            let (pk, sk) = dilithium5_keypair();
            Ok((hex::encode(pk.as_bytes()), hex::encode(sk.as_bytes())))
        }
        _ => Err(pyo3::exceptions::PyValueError::new_err("Invalid Dilithium level")),
    }
}

/// Sign message with secret key
#[pyfunction]
fn dilithium_sign(level: &str, secret_key_hex: String, message: Vec<u8>) -> PyResult<String> {
    let sk_bytes = hex::decode(secret_key_hex).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid hex SK"))?;
    match level {
        "2" => {
            let sk = Dilithium2SK::from_bytes(&sk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid SK"))?;
            let sig = dilithium2_sign(&message, &sk);
            Ok(hex::encode(sig.as_bytes()))
        }
        "3" => {
            let sk = Dilithium3SK::from_bytes(&sk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid SK"))?;
            let sig = dilithium3_sign(&message, &sk);
            Ok(hex::encode(sig.as_bytes()))
        }
        "5" => {
            let sk = Dilithium5SK::from_bytes(&sk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid SK"))?;
            let sig = dilithium5_sign(&message, &sk);
            Ok(hex::encode(sig.as_bytes()))
        }
        _ => Err(pyo3::exceptions::PyValueError::new_err("Invalid Dilithium level")),
    }
}

/// Verify signature on message with public key
#[pyfunction]
fn dilithium_verify(level: &str, public_key_hex: String, message: Vec<u8>, signature_hex: String) -> PyResult<bool> {
    let pk_bytes = hex::decode(public_key_hex).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid hex PK"))?;
    let sig_bytes = hex::decode(signature_hex).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid hex Sig"))?;
    match level {
        "2" => {
            let pk = Dilithium2PK::from_bytes(&pk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid PK"))?;
            let sig = Dilithium2Sig::from_bytes(&sig_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid Sig"))?;
            Ok(dilithium2_verify(&message, &sig, &pk))
        }
        "3" => {
            let pk = Dilithium3PK::from_bytes(&pk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid PK"))?;
            let sig = Dilithium3Sig::from_bytes(&sig_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid Sig"))?;
            Ok(dilithium3_verify(&message, &sig, &pk))
        }
        "5" => {
            let pk = Dilithium5PK::from_bytes(&pk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid PK"))?;
            let sig = Dilithium5Sig::from_bytes(&sig_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid Sig"))?;
            Ok(dilithium5_verify(&message, &sig, &pk))
        }
        _ => Err(pyo3::exceptions::PyValueError::new_err("Invalid Dilithium level")),
    }
}

/// [Preserve all prior functions: kyber_keygen/encapsulate/decapsulate, forensic_hash, merkle_root, generate_merkle_proof, verify_merkle_proof, halo2_*, etc.]

/// FENCA Rust pyo3 module
#[pymodule]
fn fenca_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(forensic_hash, m)?)?;
    m.add_function(wrap_pyfunction!(merkle_root, m)?)?;
    m.add_function(wrap_pyfunction!(generate_merkle_proof, m)?)?;
    m.add_function(wrap_pyfunction!(verify_merkle_proof, m)?)?;
    m.add_function(wrap_pyfunction!(kyber_keygen, m)?)?;
    m.add_function(wrap_pyfunction!(kyber_encapsulate, m)?)?;
    m.add_function(wrap_pyfunction!(kyber_decapsulate, m)?)?;
    m.add_function(wrap_pyfunction!(dilithium_keygen, m)?)?;
    m.add_function(wrap_pyfunction!(dilithium_sign, m)?)?;
    m.add_function(wrap_pyfunction!(dilithium_verify, m)?)?;
    m.add("__doc__", "FENCA Rust with pure ML-DSA (Dilithium) post-quantum signatures + ML-KEM (Kyber) KEM eternal")?;
    Ok(())
}
