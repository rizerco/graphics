use crate::{Image, Point, Rect};

/// Defines an image mask.
#[derive(Debug, Clone)]
pub enum Mask {
    /// A mask image that is contained within a bounding box.
    Bounded { image: Image, origin: Point<i32> },
    /// A mask image that is tiled across the canvas.
    Tiled { image: Image, offset: Point<i32> },
}

// MARK: Creation

impl Mask {
    /// Creates a new bounded mask from an image and bounds.
    /// If the image size does not match the bounds, it will
    /// be resized.
    pub fn bounded(image: Image, bounds: Rect<i32>) -> Self {
        let mut image = image;
        if image.size != bounds.size.into() {
            image.resize_nearest_neighbor(bounds.size.into());
        }
        Self::Bounded {
            image,
            origin: bounds.origin,
        }
    }
}

impl Mask {
    /// Returns the bounding box, if applicable.
    pub fn bounding_box(&self) -> Option<Rect<i32>> {
        match self {
            Mask::Bounded { image, origin } => {
                let rect = Rect {
                    origin: origin.clone(),
                    size: image.size.into(),
                };
                Some(rect)
            }
            Mask::Tiled {
                image: _,
                offset: _,
            } => None,
        }
    }
}
