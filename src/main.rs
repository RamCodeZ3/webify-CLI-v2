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
  }

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Wc { path, keep } => commands::webp::run(path, *keep),
    };

    if let Err(err) = result {
        eprintln!("{} {}", "error:".red().bold(), err);
        std::process::exit(1);
    }
}
