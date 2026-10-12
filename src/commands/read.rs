use clap::Args;
use std::error::Error;

use crate::application::operations::*;

/// Consultar una entrada en la Wallet
#[derive(Args)]
pub struct ReadArgs {}

/**
 * Consultar una entrada en la Wallet
 *
 */
pub fn run(_args: ReadArgs) -> Result<(), Box<dyn Error>> {
    let mut master_psw = String::new();
    let wallet = get_wallet(&mut master_psw)?;
    let selected_entry = select_entry(&wallet)?;
    show_credentials(&selected_entry)?;
    Ok(())
}
