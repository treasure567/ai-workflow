pub mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "passman", about = "Secure password manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new vault
    Init,
    /// Add a new entry
    Add,
    /// Retrieve an entry
    Get {
        /// Entry name
        name: String,
    },
    /// List all entry names
    List,
    /// Delete an entry
    Delete {
        /// Entry name
        name: String,
    },
    /// Rename an entry
    Rename {
        /// Current entry name
        old_name: String,
        /// New entry name
        new_name: String,
    },
    /// Change the master password
    ChangeMaster,
    /// Reset master password (destroys all data)
    ResetMaster,
    /// Launch the GUI
    Gui,
}
