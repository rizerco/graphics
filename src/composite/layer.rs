use std::borrow::Cow;

use crate::{BlendMode, Image, Mask, Point, Size};

/// Represents a layer that can be composited with
/// other layers to create a single image.
#[derive(Debug, Clone)]
pub struct Layer<'a> {
    /// The image to composite.
    pub image: Cow<'a, Image>,
    /// The position of the image on the canvas.
    pub position: Point<f32>,
    /// The size of the image on the canvas.
    pub size_on_canvas: Size<f32>,
    /// The layer’s blend mode.
    pub blend_mode: BlendMode,
    /// The layer’s opacity.
    pub opacity: f32,
    /// The layer’s masks.
    pub masks: Vec<Mask<'a>>,
}

// MARK: Creation

impl<'a> Layer<'a> {
    /// Creates a new layer for compositing.
    pub fn new(image: &'a Image, position: Point<f32>) -> Self {
        let size_on_canvas = image.size.into();
        Self {
            image: Cow::Borrowed(image),
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
            image: Cow::Owned(image),
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
    ///
    /// If the layer should be fully opaque at this location,
    /// u8::MAX is returned.
    /// If it should be fully transparent, 0 is returned,
    /// If there is no mask, u8::MAX is returned.
    pub fn mask_alpha(&self, location: Point<u32>) -> u8 {
        let mut result = u8::MAX;

        let mut location = location.into();
        for mask in self.masks.iter() {
            let color = match mask {
                Mask::Positioned(positioned_mask) => {
                    location -= positioned_mask.origin;
                    positioned_mask.image.pixel_color(location)
                }
                Mask::Tiled(tiled_mask) => {
                    location -= tiled_mask.offset;
                    let width = tiled_mask.image.size.width as i32;
                    let height = tiled_mask.image.size.height as i32;
                    if location.x < 0 {
                        location.x = width + (location.x % width);
                    }
                    if location.x >= width {
                        location.x = location.x % width;
                    }
                    if location.y < 0 {
                        location.y = height + (location.y % height);
                    }
                    if location.y >= height {
                        location.y = location.y % height;
                    }
                    tiled_mask.image.pixel_color(location)
                }
            };
            // Out of bounds.
            let Some(color) = color else {
                return 0;
            };
            let alpha = (result as u16 * color.alpha as u16) / u8::MAX as u16;
            result = alpha as u8;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use crate::{Color, Image, Mask, Point, PositionedMask, Size, TiledMask};

    use super::Layer;

    #[test]
    fn mask_alpha_no_masks() {
        let image = Image::color(
            &Color::MAGENTA,
            Size {
                width: 13,
                height: 21,
            },
        );
        let position = Point { x: 3.0, y: 5.0 };
        let layer = Layer::new(&image, position);

        let location = Point { x: 4, y: 9 };
        let alpha = layer.mask_alpha(location);

        assert_eq!(alpha, u8::MAX);
    }

    #[test]
    fn mask_alpha_positioned_mask() {
        let image = Image::color(
            &Color::MAGENTA,
            Size {
                width: 13,
                height: 21,
            },
        );
        let position = Point { x: 3.0, y: 5.0 };
        let mut layer = Layer::new(&image, position);

        let image = Image::color(
            &Color::BLACK,
            Size {
                width: 4,
                height: 3,
            },
        );
        let mask = PositionedMask {
            image: Cow::Borrowed(&image),
            origin: Point { x: 2, y: 1 },
        };
        let mask = Mask::Positioned(mask);
        layer.masks = vec![mask];

        let location = Point { x: 4, y: 3 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, u8::MAX);

        let location = Point { x: 0, y: 0 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, 0);
    }

    #[test]
    fn mask_alpha_tiled_mask() {
        let image = Image::color(
            &Color::MAGENTA,
            Size {
                width: 13,
                height: 21,
            },
        );
        let position = Point { x: 3.0, y: 5.0 };
        let mut layer = Layer::new(&image, position);

        // Setting up a checkerboard image.
        let mut image = Image::empty(Size {
            width: 2,
            height: 2,
        });
        image.set_pixel_color(Color::BLACK, Point::zero());
        image.set_pixel_color(Color::BLACK, Point { x: 1, y: 1 });

        let mask = TiledMask {
            image: Cow::Borrowed(&image),
            offset: Point { x: 1, y: 0 },
        };
        let mask = Mask::Tiled(mask);
        layer.masks = vec![mask];

        let location = Point { x: 0, y: 0 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, 0);

        let location = Point { x: 1, y: 0 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, u8::MAX);

        let location = Point { x: 2, y: 0 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, 0);

        let location = Point { x: 3, y: 0 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, u8::MAX);

        let location = Point { x: 4, y: 0 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, 0);

        let location = Point { x: 5, y: 0 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, u8::MAX);

        let location = Point { x: 0, y: 2 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, 0);

        let location = Point { x: 1, y: 2 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, u8::MAX);

        let location = Point { x: 2, y: 2 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, 0);

        let location = Point { x: 3, y: 2 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, u8::MAX);

        let location = Point { x: 4, y: 2 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, 0);

        let location = Point { x: 5, y: 2 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, u8::MAX);

        let location = Point { x: 0, y: 3 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, u8::MAX);

        let location = Point { x: 1, y: 3 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, 0);

        let location = Point { x: 2, y: 3 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, u8::MAX);

        let location = Point { x: 3, y: 3 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, 0);

        let location = Point { x: 4, y: 3 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, u8::MAX);

        let location = Point { x: 5, y: 3 };
        let alpha = layer.mask_alpha(location);
        assert_eq!(alpha, 0);
    }
}
