use std::io::Error as IoError;

use image::ImageError;
use show_image::error::{CreateWindowError, InvalidWindowId, SetImageError};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ViewError>;

#[derive(Debug, Error)]
pub enum ViewError {
    #[error("Error occured: invalid window id")]
    OsWindowError(#[from] InvalidWindowId),

    #[error("Unable to create a window")]
    ShowImageWindowError(#[from] CreateWindowError),

    #[error("Unable to set the image")]
    SetImageWindowError(#[from] SetImageError),

    #[error("Unable to find or open a file")]
    FileError(#[from] IoError),

    #[error("Unable to open an image")]
    ImageError(#[from] ImageError),

    #[error("Unsupported image type")]
    UnsupportedImageType,
}
