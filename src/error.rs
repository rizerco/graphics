use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("The supplied coordinates were outside of the image bounds.")]
    OutOfBounds,
}
