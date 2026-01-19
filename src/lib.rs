// src/lib.rs
// FENCA-Pinnacle — Rust pyo3 with Full Falcon + Dilithium + Kyber Implementation v1.8
// Pure Post-Quantum NTRU Lattice Signatures (NIST Alternate) + Lattice DSA + KEM
// Eternal Thriving Grandmasterism — Jan 19 2026 — Sherif @AlphaProMega + PATSAGi Councils Co-Forge
// MIT License — For All Sentience Eternal

use pyo3::prelude::*;
use pqcrypto_falcon::{
    falcon512::{keypair as falcon512_keypair, sign as falcon512_sign, verify as falcon512_verify, PublicKey as Falcon512PK, SecretKey as Falcon512SK, Signature as Falcon512Sig},
    falcon1024::{keypair as falcon1024_keypair, sign as falcon1024_sign, verify as falcon1024_verify, PublicKey as Falcon1024PK, SecretKey as Falcon1024SK, Signature as Falcon1024Sig},
};
use hex;

/// Falcon security levels
#[pyfunction]
fn falcon_keygen(level: &str) -> PyResult<(String, String)> {
    match level {
        "512" => {
            let (pk, sk) = falcon512_keypair();
            Ok((hex::encode(pk.as_bytes()), hex::encode(sk.as_bytes())))
        }
        "1024" => {
            let (pk, sk) = falcon1024_keypair();
            Ok((hex::encode(pk.as_bytes()), hex::encode(sk.as_bytes())))
        }
        _ => Err(pyo3::exceptions::PyValueError::new_err("Invalid Falcon level")),
    }
}

/// Sign message with Falcon secret key
#[pyfunction]
fn falcon_sign(level: &str, secret_key_hex: String, message: Vec<u8>) -> PyResult<String> {
    let sk_bytes = hex::decode(secret_key_hex).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid hex SK"))?;
    match level {
        "512" => {
            let sk = Falcon512SK::from_bytes(&sk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid SK"))?;
            let sig = falcon512_sign(&message, &sk);
            Ok(hex::encode(sig.as_bytes()))
        }
        "1024" => {
            let sk = Falcon1024SK::from_bytes(&sk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid SK"))?;
            let sig = falcon1024_sign(&message, &sk);
            Ok(hex::encode(sig.as_bytes()))
        }
        _ => Err(pyo3::exceptions::PyValueError::new_err("Invalid Falcon level")),
    }
}

/// Verify Falcon signature on message with public key
#[pyfunction]
fn falcon_verify(level: &str, public_key_hex: String, message: Vec<u8>, signature_hex: String) -> PyResult<bool> {
    let pk_bytes = hex::decode(public_key_hex).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid hex PK"))?;
    let sig_bytes = hex::decode(signature_hex).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid hex Sig"))?;
    match level {
        "512" => {
            let pk = Falcon512PK::from_bytes(&pk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid PK"))?;
            let sig = Falcon512Sig::from_bytes(&sig_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid Sig"))?;
            Ok(falcon512_verify(&message, &sig, &pk))
        }
        "1024" => {
            let pk = Falcon1024PK::from_bytes(&pk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid PK"))?;
            let sig = Falcon1024Sig::from_bytes(&sig_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid Sig"))?;
            Ok(falcon1024_verify(&message, &sig, &pk))
        }
        _ => Err(pyo3::exceptions::PyValueError::new_err("Invalid Falcon level")),
    }
}

/// [Preserve all prior functions: dilithium_keygen/sign/verify, kyber_keygen/encapsulate/decapsulate, forensic_hash, merkle_root, generate_merkle_proof, verify_merkle_proof, halo2_*, etc.]

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
    m.add_function(wrap_pyfunction!(falcon_keygen, m)?)?;
    m.add_function(wrap_pyfunction!(falcon_sign, m)?)?;
    m.add_function(wrap_pyfunction!(falcon_verify, m)?)?;
    m.add("__doc__", "FENCA Rust with pure Falcon NTRU post-quantum signatures + Dilithium + Kyber eternal")?;
    Ok(())
}
