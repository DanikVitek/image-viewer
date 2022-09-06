use std::io::Error as IoError;

use image::ImageError;
use pixels::Error as PixelsError;
use thiserror::Error;
use winit::error::OsError;

pub type Result<T> = std::result::Result<T, ViewError>;

#[derive(Debug, Error)]
pub enum ViewError {
    #[error("Unable to create a window")]
    WindowError(#[from] OsError),

    #[error("Unable to find or open a file")]
    FileError(#[from] IoError),

    #[error("Unable to open an image")]
    ImageError(#[from] ImageError),

    #[error("Primary monitor was not found")]
    NoPrimaryMonitor,

    #[error("Unable to create a pixel buffer to display the image")]
    PixelBufferError(#[from] PixelsError),

    #[error("Unsupported image type")]
    UnsupportedImageType,
}
