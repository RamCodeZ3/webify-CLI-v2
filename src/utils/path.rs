use std::env;
use std::path::PathBuf;

use super::error::AppError;

pub fn resolve(path: &str) -> Result<PathBuf, AppError> {
    if path == "." {
        env::current_dir().map_err(|source| AppError::Io {
            path: PathBuf::from("."),
            source,
        })
    } else {
        Ok(PathBuf::from(path))
    }
}
