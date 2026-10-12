use arboard::Clipboard;
use dialoguer::{Input, Password, Select, theme::ColorfulTheme};
use tabled::{Table, settings::Style};
use uuid::Uuid;

use crate::application::basics::*;
use crate::domain::wallet::{Credentials, Entry, Wallet};

/**
 * Consumir la wallet dada la contraseña maestra
 *
 */
pub fn get_wallet(mpsw: &mut String) -> Result<Wallet, String> {
    // obtener el password a traves del usuario
    let master_psw = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Contraseña maestra")
        .interact()
        .map_err(|_| "Error al capturar la contraseña")?;

    // obtener la wallert usando el password del usuario como parte de la llave
    let wallet = load_wallet(master_psw.clone())?;
    *mpsw = master_psw;
    Ok(wallet)
}

/**
 * Crear un entry a partir de las nuevas credenciales
 *
 */
pub fn create_entry() -> Result<Entry, String> {
    let title: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Servicio")
        .interact_text()
        .map_err(|_| "Error al capturar el servicio")?;

    let username: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Usuario")
        .interact_text()
        .map_err(|_| "Error al capturar el usuario")?;

    let password = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Contraseña")
        .interact()
        .map_err(|_| "Error al capturar la contraseña")?;

    // Generar un ID único (UUID v4) y armar la nueva entrada
    let new_entry = Entry {
        id: Uuid::new_v4().to_string(),
        title,
        username,
        password,
    };
    Ok(new_entry)
}

/**
 * Seleccionar una entrada de la wallet de forma interactiva
 *
 */
pub fn select_entry(wallet: &Wallet) -> Result<Entry, String> {
    // Extraer solo los títulos para mostrarlos en el menú
    let titles: Vec<String> = wallet.entries.iter().map(|e| e.title.clone()).collect();

    // Lanzar el selector interactivo
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Selecciona un servicio")
        .default(0)
        .items(&titles)
        .interact()
        .map_err(|_| "Error al capturar el servicio")?;

    let selected_entry = wallet.entries[selection].clone();
    Ok(selected_entry)
}

/**
 * Mostrar en pantalla las credenciales de la entrada seleccionada
 *
 */
pub fn show_credentials(selected_entry: &Entry) -> Result<(), String> {
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
