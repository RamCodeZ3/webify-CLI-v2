use std::fs;
use std::path::{Path, PathBuf};

use colored::Colorize;
use webp::Encoder;

use crate::error::{AppError, AppResult};

const SUPPORTED_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png"];

const DEFAULT_QUALITY: f32 = 80.0;

pub fn convert_path(path: &Path, keep_original: bool) -> AppResult<usize> {
    if !path.exists() {
        return Err(AppError::PathNotFound(path.to_path_buf()));
    }

    if path.is_file() {
        convert_single(path, keep_original)?;
        return Ok(1);
    }

    let candidates = collect_images(path)?;

    if candidates.is_empty() {
        return Err(AppError::NoImagesFound(path.to_path_buf()));
    }

    let mut converted = 0usize;

    for entry in &candidates {
        match convert_single(entry, keep_original) {
            Ok(output) => {
                println!(
                    "{} {}",
                    "convertida a".green(),
                    output.display().to_string().green().bold()
                );
                converted += 1;
            }
           Err(err) => {
                eprintln!("{} {}", "error:".red().bold(), err.to_string().red());
            }
        }
    }

    Ok(converted)
}


fn collect_images(dir: &Path) -> AppResult<Vec<PathBuf>> {
    let mut files = Vec::new();

    let entries = fs::read_dir(dir).map_err(|source| AppError::Io {
        path: dir.to_path_buf(),
        source,
    })?;

    for entry in entries {
        let entry = entry.map_err(|source| AppError::Io {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();

        if path.is_file() && has_supported_extension(&path) {
            files.push(path);
        }
    }

    Ok(files)
}

fn has_supported_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| SUPPORTED_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}


fn convert_single(path: &Path, keep_original: bool) -> AppResult<PathBuf> {
    if !has_supported_extension(path) {
        return Err(AppError::InvalidExtension(path.to_path_buf()));
    }

    let img = image::open(path).map_err(|source| AppError::Image {
        path: path.to_path_buf(),
        source,
    })?;

    let rgb = img.to_rgb8();
    let (width, height) = (rgb.width(), rgb.height());

    let encoder = Encoder::from_rgb(&rgb, width, height);
    let webp_data = encoder.encode(DEFAULT_QUALITY);

    let output_path = path.with_extension("webp");

    fs::write(&output_path, &*webp_data).map_err(|source| AppError::Io {
        path: output_path.clone(),
        source,
    })?;

    if !keep_original {
        fs::remove_file(path).map_err(|source| AppError::Io {
            path: path.to_path_buf(),
            source,
        })?;
    }

    Ok(output_path)
}
