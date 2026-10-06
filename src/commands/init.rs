use clap::Args;
use std::error::Error;

use crate::application::basics::*;
use crate::domain::wallet::{Entry, Wallet};

/**
 * El tipo derivado de Args contiene los
 * argumentos concedidos para este comando,
 * junto a configuraciones extras
 *
 */
#[derive(Args)]
pub struct InitArgs {
    #[arg(short = 'p', long)]
    password: Option<String>,
}

/**
 * la funcion run es la ejecucion del comando en si
 *
 */
pub fn run(args: InitArgs) -> Result<(), Box<dyn Error>> {
    let master_psw = args.password.unwrap_or("".to_string());

    // let wallet = Wallet { entries: vec![] };
    let wallet = Wallet {
        entries: vec![Entry {
            id: "0001".to_string(),
            title: "demo".to_string(),
            username: "user".to_string(),
            password: "pswd".to_string(),
        }],
    };

    init_wallet(master_psw, wallet)?;
    Ok(())
}
