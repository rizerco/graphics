use crate::Size;

use super::layer::Layer;

/// Represents an operation for the compositor.
#[derive(Debug)]
pub struct Operation<'a> {
    /// The layers to composite.
    pub layers: Vec<Layer<'a>>,
    /// The size of the canvas on which to composite the images.
    pub size: Size<u32>,
    /// Whether or not the final output should be premultiplied.
    pub should_premultiply: bool,
}

// CREATION

impl<'a> Operation<'a> {
    /// Creates a new operation.
    pub fn new(layers: Vec<Layer<'a>>, size: Size<u32>) -> Self {
        Self {
            layers,
            size,
            should_premultiply: false,
        }
    }
}
