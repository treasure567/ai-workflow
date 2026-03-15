use std::io::{self, Write};

use crate::models::entry::Entry;
use crate::vault::storage;
use crate::vault::vault::{init_vault, load_vault, save_vault};

pub fn cmd_init() -> Result<(), String> {
    if storage::vault_exists() {
        return Err("Vault already exists. Use 'reset-master' to start over.".to_string());
    }

    let password = prompt_master_password("Enter master password: ")?;
    let confirm = prompt_master_password("Confirm master password: ")?;

    if password != confirm {
        return Err("Passwords do not match.".to_string());
    }

    if password.len() < 8 {
        return Err("Master password must be at least 8 characters.".to_string());
    }

    init_vault(&password)?;
    println!("Vault initialized at {:?}", storage::vault_path());
    Ok(())
}

pub fn cmd_add() -> Result<(), String> {
    let master = prompt_master_password("Master password: ")?;
    let (mut vault, salt) = load_vault(&master)?;

    let name = prompt("Name: ")?;
    let username = prompt("Username: ")?;
    let password = prompt_master_password("Password: ")?;
    let url = prompt("URL: ")?;
    let notes = prompt("Notes: ")?;

    if name.is_empty() {
        return Err("Name cannot be empty.".to_string());
    }

    if vault.get(&name).is_some() {
        return Err(format!("Entry '{}' already exists.", name));
    }

    vault.add(Entry {
        name: name.clone(),
        username,
        password,
        url,
        notes,
    });

    save_vault(&vault, &master, &salt)?;
    println!("Entry '{}' added.", name);
    Ok(())
}

pub fn cmd_get(name: &str) -> Result<(), String> {
    let master = prompt_master_password("Master password: ")?;
    let (vault, _) = load_vault(&master)?;

    match vault.get(name) {
        Some(entry) => {
            println!("Name:     {}", entry.name);
            println!("Username: {}", entry.username);
            println!("Password: {}", entry.password);
            println!("URL:      {}", entry.url);
            println!("Notes:    {}", entry.notes);
            Ok(())
        }
        None => Err(format!("Entry '{}' not found.", name)),
    }
}

pub fn cmd_list() -> Result<(), String> {
    let master = prompt_master_password("Master password: ")?;
    let (vault, _) = load_vault(&master)?;

    let entries = vault.list();
    if entries.is_empty() {
        println!("No entries stored.");
    } else {
        for entry in entries {
            println!("{}", entry.name);
        }
    }
    Ok(())
}

pub fn cmd_delete(name: &str) -> Result<(), String> {
    let master = prompt_master_password("Master password: ")?;
    let (mut vault, salt) = load_vault(&master)?;

    if vault.get(name).is_none() {
        return Err(format!("Entry '{}' not found.", name));
    }

    print!("Delete '{}'? [y/N]: ", name);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("input error: {e}"))?;

    if input.trim().to_lowercase() != "y" {
        println!("Cancelled.");
        return Ok(());
    }

    vault.delete(name);
    save_vault(&vault, &master, &salt)?;
    println!("Entry '{}' deleted.", name);
    Ok(())
}

pub fn cmd_rename(old_name: &str, new_name: &str) -> Result<(), String> {
    let master = prompt_master_password("Master password: ")?;
    let (mut vault, salt) = load_vault(&master)?;

    if !vault.rename(old_name, new_name) {
        return Err(format!("Entry '{}' not found.", old_name));
    }

    save_vault(&vault, &master, &salt)?;
    println!("Renamed '{}' to '{}'.", old_name, new_name);
    Ok(())
}

pub fn cmd_change_master() -> Result<(), String> {
    let current = prompt_master_password("Current master password: ")?;
    let (vault, _) = load_vault(&current)?;

    let new_password = prompt_master_password("New master password: ")?;
    let confirm = prompt_master_password("Confirm new master password: ")?;

    if new_password != confirm {
        return Err("Passwords do not match.".to_string());
    }

    if new_password.len() < 8 {
        return Err("Master password must be at least 8 characters.".to_string());
    }

    let new_salt = crate::crypto::key_derivation::generate_salt();
    save_vault(&vault, &new_password, &new_salt)?;
    println!("Master password changed.");
    Ok(())
}

pub fn cmd_reset_master() -> Result<(), String> {
    println!("WARNING: Resetting the master password will permanently delete all stored credentials.");
    print!("Are you sure? [y/N]: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("input error: {e}"))?;

    if input.trim().to_lowercase() != "y" {
        println!("Cancelled.");
        return Ok(());
    }

    storage::delete_vault()?;

    let password = prompt_master_password("Enter new master password: ")?;
    let confirm = prompt_master_password("Confirm new master password: ")?;

    if password != confirm {
        return Err("Passwords do not match.".to_string());
    }

    if password.len() < 8 {
        return Err("Master password must be at least 8 characters.".to_string());
    }

    init_vault(&password)?;
    println!("Vault reset. All previous data has been destroyed.");
    Ok(())
}

fn prompt_master_password(message: &str) -> Result<String, String> {
    rpassword::prompt_password(message).map_err(|e| format!("failed to read password: {e}"))
}

fn prompt(message: &str) -> Result<String, String> {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("input error: {e}"))?;
    Ok(input.trim().to_string())
}
