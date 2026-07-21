use std::path::PathBuf;

use colored::Colorize;

use crate::core::favicon::FaviconGenerator;
use crate::utils::error::AppResult;
use crate::utils::path;

pub fn run(input_path: &str, name_app: &str, destination: &str) -> AppResult<()> {
    let target = path::resolve(input_path)?;

    let name_app = (!name_app.is_empty()).then(|| name_app.to_string());
    let destination = (!destination.is_empty()).then(|| PathBuf::from(destination));

    let generator = FaviconGenerator::new(target, name_app, destination)?;
    let report = generator.generate_all();

    for (item, err) in &report.failed {
        eprintln!(
            "{} {}: {}",
            "error:".red().bold(),
            item,
            err.to_string().red()
        );
    }

    for output in &report.succeeded {
        println!(
            "{} {}",
            "created".green(),
            output.display().to_string().green().bold()
        );
    }

    println!(
        "{}",
        format!(
            "A total of {} favicon files were generated",
            report.generated_count()
        )
        .green()
        .bold()
    );

    println!("\n{}", generator.html_snippet());

    Ok(())
}
