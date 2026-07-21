mod commands;
mod core;
mod utils;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(
    name = "webp_cli",
    about = "Convert images to WebP and generate favicons from the terminal"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Wc {
        path: String,
        #[arg(short = 'k', long = "keep")]
        keep: bool,
    },
    Favicon {
        path: String,
        #[arg(short = 'n', long = "name-app")]
        name_app: Option<String>,
        #[arg(short = 'd', long = "destination")]
        destination: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Wc { path, keep } => commands::webp::run(path, *keep),
        Commands::Favicon {
            path,
            name_app,
            destination,
        } => commands::favicon::run(path, name_app.as_deref(), destination.as_deref()),
    };

    if let Err(err) = result {
        eprintln!("{} {}", "error:".red().bold(), err);
        std::process::exit(1);
    }
}
