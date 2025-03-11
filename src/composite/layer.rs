use crate::{BlendMode, Image, Mask, Point, Size};

/// Represents a layer that can be composited with
/// other layers to create a single image.
#[derive(Debug, Clone)]
pub struct Layer<'a> {
    /// The image to composite.
    pub image: Either<'a, Image>,
    /// The position of the image on the canvas.
    pub position: Point<f32>,
    /// The size of the image on the canvas.
    pub size_on_canvas: Size<f32>,
    /// The layer’s blend mode.
    pub blend_mode: BlendMode,
    /// The layer’s opacity.
    pub opacity: f32,
    /// The layer’s masks.
    pub masks: Vec<&'a Mask>,
}

/// Defines a property that can be either owned or borrowed.
#[derive(Debug, Clone)]
pub enum Either<'a, T> {
    /// The owned value.
    Owned(T),
    /// The borrowed value.
    Borrowed(&'a T),
}

// MARK: Creation

impl<'a> Layer<'a> {
    /// Creates a new layer for compositing.
    pub fn new(image: &'a Image, position: Point<f32>) -> Self {
        let size_on_canvas = image.size.into();
        Self {
            image: Either::Borrowed(image),
            position,
            size_on_canvas,
            blend_mode: BlendMode::default(),
            opacity: 1.0,
            masks: Vec::new(),
        }
    }

    /// Creates a new layer with an owned image.
    pub fn new_owned(image: Image, position: Point<f32>) -> Self {
        let size_on_canvas = image.size.into();
        Self {
            image: Either::Owned(image),
            position,
            size_on_canvas,
            blend_mode: BlendMode::default(),
            opacity: 1.0,
            masks: Vec::new(),
        }
    }
}

// MARK: Utilities

impl<'a> Layer<'a> {
    /// Returns the alpha for the mask at a given location.
    /// If there is no mask, u8::MAX is returned.
    pub fn mask_alpha(&self, location: Point<u32>) -> u8 {
        let mut result = u8::MAX;

        for mask in self.masks.iter() {
            // result = ()
        }
        result
    }
}
