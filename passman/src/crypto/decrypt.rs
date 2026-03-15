use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;

use super::encrypt::EncryptedVault;

pub fn decrypt(ciphertext: &[u8], key: &[u8; 32], nonce: &[u8; 12]) -> Result<Vec<u8>, String> {
    let cipher =
        Aes256Gcm::new_from_slice(key).map_err(|e| format!("cipher init failed: {e}"))?;

    let nonce = Nonce::from_slice(nonce);

    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Invalid master password".to_string())
}

pub fn parse_encrypted_vault(
    vault: &EncryptedVault,
) -> Result<(Vec<u8>, [u8; 12], Vec<u8>), String> {
    let salt = BASE64
        .decode(&vault.salt)
        .map_err(|e| format!("invalid salt: {e}"))?;

    let nonce_vec = BASE64
        .decode(&vault.nonce)
        .map_err(|e| format!("invalid nonce: {e}"))?;

    let mut nonce = [0u8; 12];
    if nonce_vec.len() != 12 {
        return Err("invalid nonce length".to_string());
    }
    nonce.copy_from_slice(&nonce_vec);

    let ciphertext = BASE64
        .decode(&vault.ciphertext)
        .map_err(|e| format!("invalid ciphertext: {e}"))?;

    Ok((salt, nonce, ciphertext))
}
