use clap::Args;
use std::error::Error;

/// Retirar una entrada a la Wallet
#[derive(Args)]
pub struct DropArgs {}

/**
 * Retirar una entrada a la Wallet
 *
 */
pub fn run(_args: DropArgs) -> Result<(), Box<dyn Error>> {
    println!("* Aún no implementado");
    Ok(())
}
