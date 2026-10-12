use clap::Args;
use dialoguer::{Input, Password, theme::ColorfulTheme};
use std::error::Error;
use uuid::Uuid;

use crate::application::basics::*;
use crate::domain::wallet::{Entry, Wallet};

/**
 * El tipo derivado de Args contiene los
 * argumentos concedidos para este comando,
 * junto a configuraciones extras
 *
 */
#[derive(Args)]
pub struct AddArgs {}

/**
 * la funcion run es la ejecucion del comando en si
 *
 */
pub fn run(_args: AddArgs) -> Result<(), Box<dyn Error>> {
    // obtener el password a traves del usuario
    let master_psw = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Contraseña maestra")
        .interact()?;
    // obtener la wallert usando el password del usuario como parte de la llave
    let mut wallet: Wallet = load_wallet(master_psw.clone())?;

    // nuevas credenciales para almacenar en la wallet

    let title: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Secreto")
        .interact_text()
        .unwrap();

    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Usuario")
        .interact_text()
        .unwrap();

    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Contraseña")
        .interact()?;

    // Generar un ID único (UUID v4) y armar la nueva entrada
    let new_entry = Entry {
        id: Uuid::new_v4().to_string(),
        title: title.clone(),
        username,
        password,
    };

    // Agregar a la colección en memoria
    wallet.entries.push(new_entry);

    // Persistir los cambios en disco usando la función save_wallet
    save_wallet(wallet, master_psw)?;
    println!("✔ Secreto {:?} almacenado correctamente", title);

    Ok(())
}
