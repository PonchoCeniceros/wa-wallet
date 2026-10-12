use serde::{Deserialize, Serialize};
use tabled::Tabled;

/**
 * Entradas de la wallet
 * En ella se almacenan el usuario y contraseña de los diversos servicios del usuario
 *
 */
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub username: String,
    pub password: String,
}

/**
 * La Wallet es el conjunto de todas las entradas
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub entries: Vec<Entry>,
}

/**
 * Las credenciales son el nucleo de las entradas a la wallet.
 * Se usa con fines de exposicion en pantalla
 */
#[derive(Tabled)]
pub struct Credentials {
    #[tabled(rename = "Campo")]
    pub field: String,
    #[tabled(rename = "Detalle")]
    pub value: String,
}
