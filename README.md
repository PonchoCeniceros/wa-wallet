# wa²llet

Gestor de contraseñas de línea de comandos escrito en Rust. Permite guardar, listar, buscar y borrar tus credenciales de forma rápida desde la terminal, almacenándolas de forma segura en una bóveda local cifrada.

## Instalación

El binario se genera con el nombre `ww`. Para instalarlo, compílalo en release y cópialo a `/usr/local/bin/`:

```sh
cargo build --release
sudo cp "$(git rev-parse --show-toplevel)/target/release/ww" /usr/local/bin/
```

## Ubicación de la wallet

Por defecto la wallet se guarda en el directorio de datos del usuario, según el sistema operativo:

| SO      | Ruta                                              |
| ------- | ------------------------------------------------- |
| macOS   | `~/Library/Application Support/ww/wallet.db`      |
| Linux   | `~/.local/share/ww/wallet.db`                     |
| Windows | `C:\Users\<usuario>\AppData\Roaming\ww\data\wallet.db` |

La ruta se puede sobrescribir con la variable de entorno `WW_WALLET_PATH`.

Para verificar si está definida (si no imprime nada, se usa la ruta por defecto):

```sh
echo $WW_WALLET_PATH
```

## Pruebas

Para probar sin tocar tu wallet real, apunta `WW_WALLET_PATH` a un archivo temporal.

Solo para un comando:

```sh
WW_WALLET_PATH=/tmp/test.db cargo run -- init
WW_WALLET_PATH=/tmp/test.db cargo run -- add
WW_WALLET_PATH=/tmp/test.db cargo run -- read
```

Para toda la sesión de la terminal (se pierde al cerrarla):

```sh
export WW_WALLET_PATH=/tmp/test.db
cargo run -- init
cargo run -- read
```

De forma permanente, agregando la variable a `~/.zshrc`:

```sh
export WW_WALLET_PATH="$HOME/Dropbox/wallet.db"
```

y recargando la configuración con `source ~/.zshrc`.
