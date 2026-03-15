mod cli;
mod crypto;
mod gui;
mod models;
mod vault;

use clap::Parser;
use cli::commands;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init => commands::cmd_init(),
        Commands::Add => commands::cmd_add(),
        Commands::Get { ref name } => commands::cmd_get(name),
        Commands::List => commands::cmd_list(),
        Commands::Delete { ref name } => commands::cmd_delete(name),
        Commands::Rename {
            ref old_name,
            ref new_name,
        } => commands::cmd_rename(old_name, new_name),
        Commands::ChangeMaster => commands::cmd_change_master(),
        Commands::ResetMaster => commands::cmd_reset_master(),
        Commands::Gui => cmd_gui(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn cmd_gui() -> Result<(), String> {
    let master =
        rpassword::prompt_password("Master password: ").map_err(|e| format!("input error: {e}"))?;

    let (vault, salt) = vault::vault::load_vault(&master)?;
    gui::app::launch_gui(vault, master, salt)
}
