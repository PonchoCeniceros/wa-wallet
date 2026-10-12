use clap::{ArgAction, Parser, Subcommand};
mod application;
mod commands;
mod domain;

/**
 * Instancia del Cli localizada al español
 *
 */
#[derive(Parser)]
#[command(
    name = "ww",
    version,
    about = "Wallet para claves y contraseñas de usuario importantes",
    help_template = "{about}\n\nUso: {usage}\n\n{all-args}",
    subcommand_help_heading = "Comandos",
    subcommand_value_name = "COMANDO",
    disable_help_subcommand = true,
    disable_help_flag = true,
    disable_version_flag = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Muestra esta ayuda
    #[arg(short, long, action = ArgAction::Help, global = true, help_heading = "Opciones")]
    help: Option<bool>,
    /// Muestra la versión
    #[arg(short = 'V', long, action = ArgAction::Version, help_heading = "Opciones")]
    version: Option<bool>,
}

/**
 * Instancia del menu para el Cli
 *
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
