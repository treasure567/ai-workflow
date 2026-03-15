use argon2::{Algorithm, Argon2, Params, Version};
use rand::RngCore;
use zeroize::Zeroizing;

pub fn derive_key(
    master_password: &[u8],
    salt: &[u8],
) -> Result<Zeroizing<[u8; 32]>, String> {
    let params = Params::new(65536, 3, 4, Some(32))
        .map_err(|e| format!("argon2 params error: {e}"))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key = Zeroizing::new([0u8; 32]);
    argon2
        .hash_password_into(master_password, salt, key.as_mut())
        .map_err(|e| format!("key derivation failed: {e}"))?;

    Ok(key)
}

pub fn generate_salt() -> [u8; 32] {
    let mut salt = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut salt);
    salt
}
