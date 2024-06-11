use crate::{BlendMode, Image, Point, Size};

/// Represents a layer that can be composited with
/// other layers to create a single image.
#[derive(Debug, Clone)]
pub struct Layer<'a> {
    /// The image to composite.
    pub image: &'a Image,
    /// The position of the image on the canvas.
    pub position: Point<f32>,
    /// The size of the image on the canvas.
    pub size_on_canvas: Size<f32>,
    /// The layer’s blend mode.
    pub blend_mode: BlendMode,
    /// The layer’s opacity.
    pub opacity: f32,
}

// CREATION

impl<'a> Layer<'a> {
    /// Creates a new layer for compositing.
    pub fn new(image: &'a Image, position: Point<f32>) -> Self {
        let size_on_canvas = image.size.into();
        Self {
            image,
            position,
            size_on_canvas,
            blend_mode: BlendMode::default(),
            opacity: 1.0,
        }
    }
}
