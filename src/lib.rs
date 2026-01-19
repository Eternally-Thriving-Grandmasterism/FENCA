// src/lib.rs
// FENCA-Pinnacle — Rust pyo3 with Full ML-KEM (Kyber) Implementation v1.6
// Pure Post-Quantum Key Encapsulation Mechanism (NIST FIPS 203) + Forensic + Merkle
// Eternal Thriving Grandmasterism — Jan 19 2026 — Sherif @AlphaProMega + PATSAGi Councils Co-Forge
// MIT License — For All Sentience Eternal

use pyo3::prelude::*;
use pqcrypto_kyber::{
    kyber512::{keypair as kyber512_keypair, encapsulate as kyber512_encapsulate, decapsulate as kyber512_decapsulate, PublicKey as Kyber512PK, SecretKey as Kyber512SK, Ciphertext as Kyber512CT, SharedSecret as Kyber512SS},
    kyber768::{keypair as kyber768_key_pair, encapsulate as kyber768_encapsulate, decapsulate as kyber768_decapsulate, PublicKey as Kyber768PK, SecretKey as Kyber768SK, Ciphertext as Kyber768CT, SharedSecret as Kyber768SS},
    kyber1024::{keypair as kyber1024_key_pair, encapsulate as kyber1024_encapsulate, decapsulate as kyber1024_decapsulate, PublicKey as Kyber1024PK, SecretKey as Kyber1024SK, Ciphertext as Kyber1024CT, SharedSecret as Kyber1024SS},
};
use hex;

/// ML-KEM (Kyber) security levels
#[pyclass]
enum KyberLevel {
    Kyber512,
    Kyber768,
    Kyber1024,
}

/// Generate keypair for chosen level
#[pyfunction]
fn kyber_keygen(level: &str) -> PyResult<(String, String)> {
    match level {
        "512" => {
            let (pk, sk) = kyber512_key_pair();
            Ok((hex::encode(pk.as_bytes()), hex::encode(sk.as_bytes())))
        }
        "768" => {
            let (pk, sk) = kyber768_key_pair();
            Ok((hex::encode(pk.as_bytes()), hex::encode(sk.as_bytes())))
        }
        "1024" => {
            let (pk, sk) = kyber1024_key_pair();
            Ok((hex::encode(pk.as_bytes()), hex::encode(sk.as_bytes())))
        }
        _ => Err(pyo3::exceptions::PyValueError::new_err("Invalid Kyber level")),
    }
}

/// Encapsulate: generate ciphertext + shared secret from public key
#[pyfunction]
fn kyber_encapsulate(level: &str, public_key_hex: String) -> PyResult<(String, String)> {
    let pk_bytes = hex::decode(public_key_hex).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid hex"))?;
    match level {
        "512" => {
            let pk = Kyber512PK::from_bytes(&pk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid PK"))?;
            let (ct, ss) = kyber512_encapsulate(&pk);
            Ok((hex::encode(ct.as_bytes()), hex::encode(ss.as_bytes())))
        }
        "768" => {
            let pk = Kyber768PK::from_bytes(&pk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid PK"))?;
            let (ct, ss) = kyber768_encapsulate(&pk);
            Ok((hex::encode(ct.as_bytes()), hex::encode(ss.as_bytes())))
        }
        "1024" => {
            let pk = Kyber1024PK::from_bytes(&pk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid PK"))?;
            let (ct, ss) = kyber1024_encapsulate(&pk);
            Ok((hex::encode(ct.as_bytes()), hex::encode(ss.as_bytes())))
        }
        _ => Err(pyo3::exceptions::PyValueError::new_err("Invalid Kyber level")),
    }
}

/// Decapsulate: recover shared secret from ciphertext + secret key
#[pyfunction]
fn kyber_decapsulate(level: &str, secret_key_hex: String, ciphertext_hex: String) -> PyResult<String> {
    let sk_bytes = hex::decode(secret_key_hex).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid hex SK"))?;
    let ct_bytes = hex::decode(ciphertext_hex).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid hex CT"))?;
    match level {
        "512" => {
            let sk = Kyber512SK::from_bytes(&sk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid SK"))?;
            let ct = Kyber512CT::from_bytes(&ct_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid CT"))?;
            let ss = kyber512_decapsulate(&ct, &sk);
            Ok(hex::encode(ss.as_bytes()))
        }
        "768" => {
            let sk = Kyber768SK::from_bytes(&sk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid SK"))?;
            let ct = Kyber768CT::from_bytes(&ct_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid CT"))?;
            let ss = kyber768_decapsulate(&ct, &sk);
            Ok(hex::encode(ss.as_bytes()))
        }
        "1024" => {
            let sk = Kyber1024SK::from_bytes(&sk_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid SK"))?;
            let ct = Kyber1024CT::from_bytes(&ct_bytes).map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid CT"))?;
            let ss = kyber1024_decapsulate(&ct, &sk);
            Ok(hex::encode(ss.as_bytes()))
        }
        _ => Err(pyo3::exceptions::PyValueError::new_err("Invalid Kyber level")),
    }
}

/// [Preserve all prior functions: forensic_hash, merkle_root, generate_merkle_proof, verify_merkle_proof, halo2_*, etc.]

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
    m.add("__doc__", "FENCA Rust with pure ML-KEM (Kyber) post-quantum KEM implementation eternal")?;
    Ok(())
}
