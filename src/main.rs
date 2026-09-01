mod commands;
mod core;
mod utils;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(
    name = "webify",
    about = "Convert images to WebP and generate favicons from the terminal"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert any type of image to the WebP format.
    #[command(about = "Convert any type of image to the WebP format.")]
    Wc {
        /// Path to the target image or directory
        path: String,

        /// If enabled, the original image will remain unchanged
        #[arg(
            short = 'k',
            long = "keep",
            help = "If enabled, the original image will remain unchanged"
        )]
        keep: bool,
    },

    /// Generate favicons from an image
    #[command(about = "Generate favicons from an image")]
    Favicon {
        /// Path to the source image
        path: String,

        /// Name for the app or web
        #[arg(
            short = 'n',
            long = "name-app",
            help = "Name for the app or web"
        )]
        name_app: Option<String>,

        /// Manually specify where you want the favicons to be saved
        #[arg(
            short = 'd',
            long = "destination",
            help = "Manually specify where you want the favicons to be saved"
        )]
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
