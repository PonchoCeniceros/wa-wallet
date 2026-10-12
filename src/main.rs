use clap::{Parser, Subcommand};

mod application;
mod commands;
mod domain;

/**
 * Instancia del Cli
 *
 */
#[derive(Parser)]
#[command(
    name = "wwl",
    version,
    about = "Wallet para claves y contraseñas de usuario importantes"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/**
 * Instancia del menu para el Cli
 */
#[derive(Subcommand)]
enum Commands {
    Greet(commands::greet::GreetArgs),
    Init(commands::init::InitArgs),
    Load(commands::load::LoadArgs),
    // List(commands::list::ListArgs),
    Add(commands::add::AddArgs),
    // Read(commands::read::ReadArgs),
    // Drop(commands::drop::DropArgs),
}

/**
 *  Logica de negocio del Cli
 *
 *  [Consola] cargo run -- greet -n Giovanny
 *     │
 *     ▼
 *  1. Cli::parse() recupera los argumentos de la terminal.
 *     │
 *     ▼
 *  2. El enum Commands::Greet(args) recibe 'GreetArgs { username: Some("Giovanny") }'.
 *     │
 *     ▼
 *  3. El match redirige hacia commands::greet::run(args).
 *     │
 *     ▼
 *  4. Tu función run imprime "Hello Giovanny" y retorna Ok(()).
 *
 */
fn main() {
    /*
     *
     */
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Greet(args) => commands::greet::run(args),
        Commands::Init(args) => commands::init::run(args),
        Commands::Load(args) => commands::load::run(args),
        Commands::Add(args) => commands::add::run(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
