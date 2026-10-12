use clap::Args;
use std::error::Error;

use crate::application::operations::*;

/**
 * El tipo derivado de Args contiene los
 * argumentos concedidos para este comando,
 * junto a configuraciones extras
 *
 */
#[derive(Args)]
pub struct ReadArgs {}

/**
 * la funcion run es la ejecucion del comando en si
 *
 */
pub fn run(_args: ReadArgs) -> Result<(), Box<dyn Error>> {
    let mut master_psw = String::new();
    let wallet = get_wallet(&mut master_psw)?;
    let selected_entry = select_entry(&wallet)?;
    show_credentials(&selected_entry)?;
    Ok(())
}
