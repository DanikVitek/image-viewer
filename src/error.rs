use show_image::winit::error::OsError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ViewError>;

#[derive(Debug, Error)]
pub enum ViewError {
    #[error("Unable to create window")]
    WindowError(#[from] OsError),
}