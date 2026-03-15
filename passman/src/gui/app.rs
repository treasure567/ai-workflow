use std::sync::Mutex;

use serde::Serialize;
use tauri::State;

use crate::models::entry::Entry;
use crate::vault::vault::{save_vault, Vault};

pub struct AppState {
    pub vault: Mutex<Vault>,
    pub master_password: Mutex<String>,
    pub salt: Mutex<Vec<u8>>,
}

#[derive(Serialize)]
pub struct EntryResponse {
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
}

impl From<&Entry> for EntryResponse {
    fn from(e: &Entry) -> Self {
        EntryResponse {
            name: e.name.clone(),
            username: e.username.clone(),
            password: e.password.clone(),
            url: e.url.clone(),
            notes: e.notes.clone(),
        }
    }
}

#[tauri::command]
pub fn get_entries(state: State<AppState>) -> Vec<EntryResponse> {
    let vault = state.vault.lock().unwrap();
    vault.list().iter().map(EntryResponse::from).collect()
}

#[tauri::command]
pub fn add_entry(
    state: State<AppState>,
    name: String,
    username: String,
    password: String,
    url: String,
    notes: String,
) -> Result<(), String> {
    let mut vault = state.vault.lock().unwrap();

    if vault.get(&name).is_some() {
        return Err(format!("Entry '{}' already exists.", name));
    }

    vault.add(Entry {
        name,
        username,
        password,
        url,
        notes,
    });

    let master = state.master_password.lock().unwrap();
    let salt = state.salt.lock().unwrap();
    save_vault(&vault, &master, &salt)
}

#[tauri::command]
pub fn update_entry(
    state: State<AppState>,
    old_name: String,
    name: String,
    username: String,
    password: String,
    url: String,
    notes: String,
) -> Result<(), String> {
    let mut vault = state.vault.lock().unwrap();

    let entry = vault
        .entries
        .iter_mut()
        .find(|e| e.name.to_lowercase() == old_name.to_lowercase())
        .ok_or_else(|| format!("Entry '{}' not found.", old_name))?;

    entry.name = name;
    entry.username = username;
    entry.password = password;
    entry.url = url;
    entry.notes = notes;

    let master = state.master_password.lock().unwrap();
    let salt = state.salt.lock().unwrap();
    save_vault(&vault, &master, &salt)
}

#[tauri::command]
pub fn delete_entry(state: State<AppState>, name: String) -> Result<(), String> {
    let mut vault = state.vault.lock().unwrap();

    if !vault.delete(&name) {
        return Err(format!("Entry '{}' not found.", name));
    }

    let master = state.master_password.lock().unwrap();
    let salt = state.salt.lock().unwrap();
    save_vault(&vault, &master, &salt)
}

#[tauri::command]
pub fn rename_entry(
    state: State<AppState>,
    old_name: String,
    new_name: String,
) -> Result<(), String> {
    let mut vault = state.vault.lock().unwrap();

    if !vault.rename(&old_name, &new_name) {
        return Err(format!("Entry '{}' not found.", old_name));
    }

    let master = state.master_password.lock().unwrap();
    let salt = state.salt.lock().unwrap();
    save_vault(&vault, &master, &salt)
}

pub fn launch_gui(vault: Vault, master_password: String, salt: Vec<u8>) -> Result<(), String> {
    tauri::Builder::default()
        .manage(AppState {
            vault: Mutex::new(vault),
            master_password: Mutex::new(master_password),
            salt: Mutex::new(salt),
        })
        .invoke_handler(tauri::generate_handler![
            get_entries,
            add_entry,
            update_entry,
            delete_entry,
            rename_entry,
        ])
        .run(tauri::generate_context!())
        .map_err(|e| format!("GUI error: {e}"))
}

