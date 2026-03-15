use serde::{Deserialize, Serialize};

use crate::crypto::decrypt::{decrypt, parse_encrypted_vault};
use crate::crypto::encrypt::{build_encrypted_vault, encrypt};
use crate::crypto::key_derivation::{derive_key, generate_salt};
use crate::models::entry::Entry;
use crate::vault::storage;

#[derive(Serialize, Deserialize)]
pub struct Vault {
    pub entries: Vec<Entry>,
}

impl Vault {
    pub fn new() -> Self {
        Vault {
            entries: Vec::new(),
        }
    }

    pub fn add(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn get(&self, name: &str) -> Option<&Entry> {
        self.entries
            .iter()
            .find(|e| e.name.to_lowercase() == name.to_lowercase())
    }

    pub fn list(&self) -> &[Entry] {
        &self.entries
    }

    pub fn delete(&mut self, name: &str) -> bool {
        let len = self.entries.len();
        self.entries
            .retain(|e| e.name.to_lowercase() != name.to_lowercase());
        self.entries.len() != len
    }

    pub fn rename(&mut self, old_name: &str, new_name: &str) -> bool {
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|e| e.name.to_lowercase() == old_name.to_lowercase())
        {
            entry.name = new_name.to_string();
            true
        } else {
            false
        }
    }
}

pub fn init_vault(master_password: &str) -> Result<(), String> {
    if storage::vault_exists() {
        return Err("Vault already exists. Delete it first or use 'reset-master'.".to_string());
    }

    let vault = Vault::new();
    let salt = generate_salt();

    save_vault(&vault, master_password, &salt)
}

pub fn load_vault(master_password: &str) -> Result<(Vault, Vec<u8>), String> {
    let encrypted = storage::read_vault()?;
    let (salt, nonce, ciphertext) = parse_encrypted_vault(&encrypted)?;

    let key = derive_key(master_password.as_bytes(), &salt)?;
    let plaintext = decrypt(&ciphertext, &key, &nonce)?;

    let vault: Vault =
        serde_json::from_slice(&plaintext).map_err(|e| format!("corrupted vault data: {e}"))?;

    Ok((vault, salt))
}

pub fn save_vault(vault: &Vault, master_password: &str, salt: &[u8]) -> Result<(), String> {
    let plaintext =
        serde_json::to_vec(vault).map_err(|e| format!("serialization failed: {e}"))?;

    let key = derive_key(master_password.as_bytes(), salt)?;
    let (ciphertext, nonce) = encrypt(&plaintext, &key)?;

    let encrypted = build_encrypted_vault(salt, &nonce, &ciphertext);
    storage::write_vault(&encrypted)
}
