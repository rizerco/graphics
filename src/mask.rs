use crate::{Image, Rect};

/// Defines an image mask.
pub trait Mask {
    /// The image that represents the mask.
    fn image(&self) -> &Image;
    /// The bounding box of the mask.
    fn bounding_box(&self) -> Rect<i32>;
}
