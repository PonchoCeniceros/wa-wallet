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
    name = "ww",
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
    Init(commands::init::InitArgs),
    Add(commands::add::AddArgs),
    Read(commands::read::ReadArgs),
    Drop(commands::drop::DropArgs),
}

/**
 *  Logica de negocio del Cli
 *
 */
fn main() {
    /*
     *
     */
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init(args) => commands::init::run(args),
        Commands::Add(args) => commands::add::run(args),
        Commands::Read(args) => commands::read::run(args),
        Commands::Drop(args) => commands::drop::run(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
