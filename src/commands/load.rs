use arboard::Clipboard;
use clap::Args;
use dialoguer::Select;
use dialoguer::{Password, theme::ColorfulTheme};
use std::error::Error;
use tabled::{Table, settings::Style};

use crate::application::basics::*;
use crate::domain::wallet::Credentials;

/**
 * El tipo derivado de Args contiene los
 * argumentos concedidos para este comando,
 * junto a configuraciones extras
 *
 */
#[derive(Args)]
pub struct LoadArgs {}

/**
 * la funcion run es la ejecucion del comando en si
 *
 */
pub fn run(_args: LoadArgs) -> Result<(), Box<dyn Error>> {
    // obtener el password a traves del usuario
    let master_psw = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Contraseña maestra")
        .interact()?;

    // obtener la wallert usando el password del usuario como parte de la llave
    let wallet = load_wallet(master_psw)?;

    // Extraer solo los títulos para mostrarlos en el menú
    let titles: Vec<String> = wallet.entries.iter().map(|e| e.title.clone()).collect();

    // Lanzar el selector interactivo
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Selecciona un servicio")
        .default(0)
        .items(&titles)
        .interact()?;
    let selected_entry = &wallet.entries[selection];

    // Intentar copiar la contraseña al portapapeles
    let mut clipboard =
        Clipboard::new().map_err(|_| "No se pudo acceder al portapapeles del sistema")?;
    clipboard
        .set_text(selected_entry.password.clone())
        .map_err(|_| "No se pudo copiar la contraseña al portapapeles")?;

    // mostrar el resultado
    let cards = vec![
        Credentials {
            field: "Servicio".into(),
            value: selected_entry.title.clone(),
        },
        Credentials {
            field: "Usuario".into(),
            value: selected_entry.username.clone(),
        },
        Credentials {
            field: "Contraseña".into(),
            value: "[ Copiada al portapapeles ]".into(),
        },
    ];
    let mut table = Table::new(cards);
    table.with(Style::rounded());
    println!("✔ Credencial Seleccionada:");
    println!("{}", table);
    Ok(())
}
