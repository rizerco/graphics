use std::borrow::Cow;

use crate::{Image, Point, Rect};

/// Defines a mask.
#[derive(Debug, Clone)]
pub enum Mask<'a> {
    /// A positioned mask.
    Positioned(PositionedMask<'a>),
    /// A tiled mask.
    Tiled(TiledMask),
}

pub trait Flamble: Clone + std::fmt::Debug {}

/// Defines a bounded mask.
pub trait BoundedMask {
    /// The image that represents the mask.
    fn image(&self) -> &Image;
    /// The bounding box of the mask.
    fn bounding_box(&self) -> Rect<i32>;
}

#[derive(Debug, Clone)]
/// A mask image that is contained within a bounding box.
pub struct PositionedMask<'a> {
    pub image: Cow<'a, Image>,
    pub origin: Point<i32>,
}

#[derive(Debug, Clone)]
/// A mask image that is tiled across the canvas.
pub struct TiledMask {
    pub image: Image,
    pub offset: Point<i32>,
}

// MARK: Creation

impl<'a> PositionedMask<'a> {
    /// Creates a new bounded mask from an image and bounds.
    /// If the image size does not match the bounds, it will
    /// be resized.
    pub fn bounded(image: Image, bounds: Rect<i32>) -> Self {
        let mut image = image;
        if image.size != bounds.size.into() {
            image.resize_nearest_neighbor(bounds.size.into());
        }
        Self {
            image: Cow::Owned(image),
            origin: bounds.origin,
        }
    }
}

impl<'a> BoundedMask for PositionedMask<'a> {
    /// Returns the bounding box, if applicable.
    fn bounding_box(&self) -> Rect<i32> {
        Rect {
            origin: self.origin.clone(),
            size: self.image.size.into(),
        }
    }

    fn image(&self) -> &Image {
        &self.image
    }
}
