use thiserror::Error;

#[derive(Error, Debug)]
pub enum ImageError {
    #[error("The supplied coordinates were outside of the image bounds.")]
    OutOfBounds,
}

#[derive(Error, Debug)]
pub enum TrimError {
    #[error("Container is outside of the image bounds.")]
    OutOfBounds,
    #[error("The image can’t be trimmed because it is fully transparent.")]
    FullyTransparent,
    #[error("An error occurred when cropping.")]
    Crop(#[from] anyhow::Error),
}
