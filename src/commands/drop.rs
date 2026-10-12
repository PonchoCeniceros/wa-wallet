use clap::Args;
use std::error::Error;

/**
 * El tipo derivado de Args contiene los
 * argumentos concedidos para este comando,
 * junto a configuraciones extras
 *
 */
#[derive(Args)]
pub struct DropArgs {}

/**
 * la funcion run es la ejecucion del comando en si
 *
 */
pub fn run(_args: DropArgs) -> Result<(), Box<dyn Error>> {
    println!("* Aún no implementado");
    Ok(())
}
