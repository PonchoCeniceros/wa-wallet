use clap::Args;
use std::error::Error;

use crate::application::basics::*;
use crate::application::operations::*;

/// Realizar una entrada a la Wallet
#[derive(Args)]
pub struct AddArgs {}

/**
 * Realizar una entrada a la Wallet
 */
pub fn run(_args: AddArgs) -> Result<(), Box<dyn Error>> {
    let mut master_psw = String::new();
    let mut wallet = get_wallet(&mut master_psw)?;
    let entry = create_entry()?;

    // almacenar la nueva entry en la wallet
    let title = entry.title.clone();
    wallet.entries.push(entry);
    save_wallet(wallet, master_psw)?;
    println!("✔ Secreto {:?} almacenado correctamente", title);
    Ok(())
}
