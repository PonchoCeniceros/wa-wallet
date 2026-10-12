use serde::{Deserialize, Serialize};
use tabled::Tabled;

/**
 * Entradas a mi wallet
 * Lo que vive dentro del payload cifrado
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub username: String,
    pub password: String,
}

/**
 * Coleccion de las entradas a mi wallet
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Wallet {
    pub entries: Vec<Entry>,
}

/**
 * Variante de la entry para manejar las
 * credenciales al consultarlas
 */
#[derive(Tabled)]
pub struct Credentials {
    #[tabled(rename = "Campo")]
    pub field: String,
    #[tabled(rename = "Detalle")]
    pub value: String,
}
