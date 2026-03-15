use std::fs;
use std::path::PathBuf;

use crate::crypto::encrypt::EncryptedVault;

pub fn vault_dir() -> PathBuf {
    let home = dirs_impl();
    home.join(".passman")
}

pub fn vault_path() -> PathBuf {
    vault_dir().join("vault.enc")
}

pub fn vault_exists() -> bool {
    vault_path().exists()
}

pub fn ensure_vault_dir() -> Result<(), String> {
    let dir = vault_dir();
    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("failed to create vault directory: {e}"))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&dir, fs::Permissions::from_mode(0o700))
                .map_err(|e| format!("failed to set directory permissions: {e}"))?;
        }
    }
    Ok(())
}

pub fn write_vault(encrypted: &EncryptedVault) -> Result<(), String> {
    ensure_vault_dir()?;

    let path = vault_path();
    let tmp_path = path.with_extension("tmp");

    let data =
        serde_json::to_string_pretty(encrypted).map_err(|e| format!("serialization failed: {e}"))?;

    fs::write(&tmp_path, &data).map_err(|e| format!("failed to write temp file: {e}"))?;
    fs::rename(&tmp_path, &path).map_err(|e| format!("failed to rename vault file: {e}"))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
            .map_err(|e| format!("failed to set file permissions: {e}"))?;
    }

    Ok(())
}

pub fn read_vault() -> Result<EncryptedVault, String> {
    let path = vault_path();
    if !path.exists() {
        return Err("Vault not found. Run 'passman init' first.".to_string());
    }

    let data = fs::read_to_string(&path).map_err(|e| format!("failed to read vault: {e}"))?;

    serde_json::from_str(&data).map_err(|e| format!("failed to parse vault: {e}"))
}

pub fn delete_vault() -> Result<(), String> {
    let path = vault_path();
    if path.exists() {
        fs::remove_file(&path).map_err(|e| format!("failed to delete vault: {e}"))?;
    }
    Ok(())
}

fn dirs_impl() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home);
        }
    }

    #[cfg(not(target_os = "macos"))]
    {
        if let Some(home) = std::env::var_os("HOME") {
            return PathBuf::from(home);
        }
    }

    PathBuf::from(".")
}
