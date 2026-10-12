use clap::Args;
use dialoguer::{Password, theme::ColorfulTheme};
use std::error::Error;

use crate::application::basics::*;
use crate::domain::wallet::Wallet;

/// Inicializar la Wallet con una contraseña maestra
#[derive(Args)]
pub struct InitArgs {}

/**
 * Inicializar la Wallet con una contraseña maestra
 *
 */
pub fn run(_args: InitArgs) -> Result<(), Box<dyn Error>> {
    // obtener el password a traves del usuario
    let master_psw = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Contraseña maestra")
        .with_confirmation(
            "Confirma la contraseña maestra",
            "La contraseña no coincide",
        )
        .interact()
        .map_err(|_| "Error al capturar la contraseña")?;

    // crear una wallet limpia
    let wallet = Wallet { entries: vec![] };
    init_wallet(master_psw, wallet)?;
    Ok(())
}
