use clap::Args;
use std::error::Error;

/**
 * El tipo derivado de Args contiene los
 * argumentos concedidos para este comando,
 * junto a configuraciones extras
 *
 */
#[derive(Args)]
pub struct GreetArgs {
    #[arg(short = 'n', long)]
    username: Option<String>,
}

/**
 * la funcion run es la ejecucion del comando en si
 *
 */
pub fn run(args: GreetArgs) -> Result<(), Box<dyn Error>> {
    let def_usrn = String::from("");
    let username = args.username.unwrap_or(def_usrn);
    println!("Hello {}", username);
    Ok(())
}
