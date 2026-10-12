use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use argon2::Argon2;
use directories::ProjectDirs;
use rand::*;
use std::path::PathBuf;

use crate::domain::wallet::Wallet;

const WALLET_FILE: &str = "wallet.db";
const WALLET_ENV: &str = "WW_WALLET_PATH";

/**
 * Resolver la ruta de la wallet
 * prioridad: variable de entorno WW_WALLET_PATH > directorio de datos del SO
 */
fn wallet_path() -> Result<PathBuf, String> {
    if let Some(custom) = std::env::var_os(WALLET_ENV) {
        return Ok(PathBuf::from(custom));
    }

    let dirs = ProjectDirs::from("", "", "ww")
        .ok_or("No se pudo determinar el directorio home del usuario")?;
    Ok(dirs.data_dir().join(WALLET_FILE))
}

/**
 * Crear por primera vez la instancia de mi Wallet
 *
 */
pub fn init_wallet(master_psw: String, base_wallet: Wallet) -> Result<(), String> {
    // generacion de los parametros random
    let mut rng = rand::thread_rng();
    let salt: [u8; 16] = rng.r#gen();
    let nonce: [u8; 12] = rng.r#gen();
    let mut key = [0u8; 32];

    // generacion de mi key a partir de mi llave maestra
    Argon2::default()
        .hash_password_into(master_psw.as_bytes(), &salt, &mut key)
        .map_err(|_| "Error al derivar la clave".to_string())?;

    // serializar Wallet a bytes JSON
    let wallet_bytes = serde_json::to_vec(&base_wallet)
        .map_err(|_| "Error al serializar la wallet".to_string())?;

    // cifrar con AES-256-GCM
    let cipher = Aes256Gcm::new_from_slice(&key).expect("Error al crear el cifrador");
    let nonce_bound = Nonce::from_slice(&nonce);

    let ciphertext = cipher
        .encrypt(nonce_bound, wallet_bytes.as_ref())
        .map_err(|_| "Error al cifrar el contenido".to_string())?;

    // guardar el archivo final
    let mut file_content = Vec::new();
    file_content.extend_from_slice(&salt);
    file_content.extend_from_slice(&nonce);
    file_content.extend_from_slice(&ciphertext);

    let path = wallet_path()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|_| format!("No se pudo crear el directorio {}", parent.display()))?;
    }

    std::fs::write(&path, file_content)
        .map_err(|_| format!("Error al escribir la wallet en {}", path.display()))?;

    Ok(())
}

/**
 * Cargar en memoria la instancia de mi Wallet
 *
 */
pub fn load_wallet(master_psw: String) -> Result<Wallet, String> {
    // Leer el archivo binario completo
    let path = wallet_path()?;
    let file_bytes = std::fs::read(&path).map_err(|_| {
        format!(
            "No se pudo encontrar o abrir la wallet en {}",
            path.display()
        )
    })?;

    // Validar tamaño mínimo (16 bytes Salt + 12 bytes Nonce)
    if file_bytes.len() < 28 {
        return Err("El archivo de la wallet está corrupto o incompleto".to_string());
    }

    // Particionar la cabecera (Header) y el Payload
    let salt = &file_bytes[0..16];
    let nonce = &file_bytes[16..28];
    let ciphertext = &file_bytes[28..];

    // Re-derivar la clave de 32 bytes usando el Salt guardado
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(master_psw.as_bytes(), salt, &mut key)
        .map_err(|_| "Error al derivar la clave".to_string())?;

    // Configurar AES-256-GCM y descifrar
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|_| "Error al instanciar el cifrador".to_string())?;
    let nonce_bound = Nonce::from_slice(nonce);

    let decrypted_bytes = cipher
        .decrypt(nonce_bound, ciphertext)
        .map_err(|_| "Contraseña maestra incorrecta o datos alterados".to_string())?;

    // Deserializar el JSON a la struct Wallet
    let wallet: Wallet = serde_json::from_slice(&decrypted_bytes)
        .map_err(|_| "Error al parsear el contenido de la wallet".to_string())?;

    Ok(wallet)
}

/**
 * Persistir los cambios de la Wallet en disco
 *
 */
pub fn save_wallet(wallet: Wallet, master_psw: String) -> Result<(), String> {
    // Leer el archivo existente para extraer el Salt original (primeros 16 bytes)
    let path = wallet_path()?;
    let file_bytes = std::fs::read(&path).map_err(|_| {
        format!(
            "No se pudo acceder a la wallet en {} para guardar los cambios",
            path.display()
        )
    })?;

    if file_bytes.len() < 16 {
        return Err("El archivo de la wallet está corrupto".to_string());
    }

    let salt = &file_bytes[0..16];

    // Generar un NUEVO Nonce fresco (12 bytes) para esta re-escritura
    let mut rng = rand::thread_rng();
    let new_nonce: [u8; 12] = rng.r#gen();

    // Re-derivar la clave de 32 bytes
    let mut key = [0u8; 32];
    Argon2::default()
        .hash_password_into(master_psw.as_bytes(), salt, &mut key)
        .map_err(|_| "Error al derivar la clave".to_string())?;

    // Serializar el struct Wallet actualizado a JSON
    let json_bytes = serde_json::to_vec(&wallet)
        .map_err(|_| "Error al serializar el contenido de la wallet".to_string())?;

    // Cifrar con AES-256-GCM
    let cipher = Aes256Gcm::new_from_slice(&key)
        .map_err(|_| "Error al instanciar el cifrador".to_string())?;
    let nonce_bound = Nonce::from_slice(&new_nonce);

    let ciphertext = cipher
        .encrypt(nonce_bound, json_bytes.as_ref())
        .map_err(|_| "Error al cifrar los nuevos datos".to_string())?;

    // Empaquetar el buffer final: [Salt original] + [Nuevo Nonce] + [Nuevo Ciphertext]
    let mut buffer = Vec::new();
    buffer.extend_from_slice(salt);
    buffer.extend_from_slice(&new_nonce);
    buffer.extend_from_slice(&ciphertext);

    // Sobrescribir en disco
    std::fs::write(&path, buffer)
        .map_err(|_| format!("Error al escribir la wallet en {}", path.display()))?;

    Ok(())
}
