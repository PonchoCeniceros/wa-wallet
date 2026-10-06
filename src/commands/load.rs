use clap::Args;
use std::error::Error;

use crate::application::basics::*;

/**
 * El tipo derivado de Args contiene los
 * argumentos concedidos para este comando,
 * junto a configuraciones extras
 *
 */
#[derive(Args)]
pub struct LoadArgs {
    #[arg(short = 'p', long)]
    password: Option<String>,
}

/**
 * la funcion run es la ejecucion del comando en si
 *
 */
pub fn run(args: LoadArgs) -> Result<(), Box<dyn Error>> {
    let master_psw = args.password.unwrap_or("".to_string());
    let wallet = load_wallet(master_psw)?;
    println!("{:?}", wallet);
    Ok(())
}
