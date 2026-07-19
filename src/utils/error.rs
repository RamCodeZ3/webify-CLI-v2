use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found path: {0}")]
    PathNotFound(PathBuf),

    #[error("Not found images in the folder : {0}")]
    NoImagesFound(PathBuf),

    #[error("Error of input/output in'{path}': {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Error proccesing image'{path}': {source}")]
    Image {
        path: PathBuf,
        #[source]
        source: image::ImageError,
    },

    #[error("The'{0}' it does not have a valid file extension")]
    InvalidExtension(PathBuf),
}

pub type AppResult<T> = Result<T, AppError>;
