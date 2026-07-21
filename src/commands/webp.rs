use colored::Colorize;

use crate::core::webp;
use crate::utils::error::AppError;
use crate::utils::path;

pub fn run(input_path: &str, keep: bool) -> Result<(), AppError> {
    let target = path::resolve(input_path)?;
    let report = webp::convert_path(&target, keep)?;

    for (path, err) in &report.failed {
        eprintln!(
            "{} {}: {}",
            "error:".red().bold(),
            path.display(),
            err.to_string().red()
        );
    }

    for output in &report.succeeded {
        println!(
            "{} {}",
            "converted to".green(),
            output.display().to_string().green().bold()
        );
    }

    println!(
        "{}",
        format!(
            "A total of {} images were converted to webp",
            report.converted_count()
        )
        .green()
        .bold()
    );

    Ok(())
}
