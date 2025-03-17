use std::cmp::min;

use rayon::prelude::*;

use crate::{BlendMode, Color, Image, Mask, Point};

use super::blend::{self, RgbaColor};
use super::operation::Operation;
use super::Layer;

/// Composites multiple images together and returns the result.
pub fn composite(operation: &Operation) -> Image {
    let mut output = Image::empty(operation.size);

    for layer in operation.layers.iter() {
        draw_layer_over_image(&mut output, &layer);
    }

    output
}

/// Draws a layer over an image.
pub fn draw_layer_over_image(image: &mut Image, layer: &Layer) {
    let location = layer.position.rounded();
    let start_x = if location.x < 0 { 0 } else { location.x as u32 };
    if start_x >= image.size.width {
        return;
    }

    let layer_size = layer.image.size;
    let layer_bytes_per_row = layer.image.bytes_per_row;

    let pixel_ratio_x = (layer_size.width as f32 / layer.size_on_canvas.width).round();
    let pixel_ratio_y = (layer_size.height as f32 / layer.size_on_canvas.height).round();

    let end_x = layer.size_on_canvas.width.round() as i32 + location.x;
    if end_x <= 0 {
        return;
    }
    let end_x = end_x as u32;
    let end_x = min(image.size.width, end_x);
    let required_width = (end_x - start_x) as usize;

    let start_y = if location.y < 0 { 0 } else { location.y as u32 };
    if start_y >= image.size.height {
        return;
    }
    let end_y = layer.size_on_canvas.height.round() as i32 + location.y;
    if end_y <= 0 {
        return;
    }
    let end_y = end_y as u32;
    let end_y = min(image.size.height, end_y);
    let required_height = end_y - start_y;

    let target_y_offset = if location.y < 0 { 0 } else { location.y as u32 };

    let x_offset = if location.x < 0 {
        location.x.abs() as usize * 4
    } else {
        0
    };

    image
        .data
        .par_chunks_mut(image.bytes_per_row as usize)
        .enumerate()
        .skip(target_y_offset as usize) // Skip rows outside the target area
        .take(required_height as usize) // Only process the necessary rows
        .for_each(|(y, row)| {
            let y = y as u32;
            let y_position = y as i32 - location.y;
            let y_position = (y_position as f32 * pixel_ratio_y).floor() as u32;
            let offset = (y_position * layer_bytes_per_row) as usize;

            for x in 0..required_width {
                let alpha = layer.mask_alpha(Point {
                    x: x as u32,
                    y: y - target_y_offset,
                });
                if alpha == 0 {
                    continue;
                }
                let mask_opacity = alpha as f32 / u8::MAX as f32;
                let x = x * 4;
                let x_position = x + x_offset;
                let x_position = (x_position as f32 * pixel_ratio_x).floor() as usize;
                let start = offset + x_position;
                let blend_color = pixel_data(&layer.image.data, start);
                let blend_color: Color = blend_color.into();

                let start = start_x as usize * 4 + x;
                let base_color = pixel_data(&row, start);
                let mut base_color: Color = base_color.into();

                blend_colors(
                    &mut base_color,
                    &blend_color,
                    layer.blend_mode,
                    layer.opacity * mask_opacity,
                );

                row[start + 0] = base_color.red;
                row[start + 1] = base_color.green;
                row[start + 2] = base_color.blue;
                row[start + 3] = base_color.alpha;
            }
        });
}

impl Image {
    /// Composites the provided image over this image.
    pub fn composite_image_over(
        &mut self,
        image: &Image,
        location: Point<i32>,
        options: &CompositeOperationOptions,
    ) {
        let mut layer = Layer::new(image, location.into());
        layer.blend_mode = options.blend_mode.to_owned();
        layer.masks = options.masks.to_owned();
        draw_layer_over_image(self, &layer);
    }
}

/// Retrieves the pixel data from a given location in a vector of RGBA bytes.
fn pixel_data(source: &[u8], offset: usize) -> [u8; 4] {
    source
        .get(offset..(offset + 4))
        .and_then(|data| data.try_into().ok())
        .unwrap_or([u8::MAX, 0, 0, u8::MAX])
    // .unwrap_or_default()
}

/// Blends one colour with another.
fn blend_colors(color: &mut Color, blend_color: &Color, blend_mode: BlendMode, opacity: f32) {
    if color.alpha == 0 && blend_color.alpha == 0 {
        return;
    };

    let mut base_rgba = blend::RgbaColor::from(color);
    let mut blend_rgba = blend::RgbaColor::from(blend_color);
    let mut base_rgb = blend::RgbColor::from_rgba_color(&base_rgba);
    let blend_rgb = blend::RgbColor::from_rgba_color(&blend_rgba);

    match blend_mode {
        BlendMode::Addition => blend::addition(&mut base_rgb, &blend_rgb),
        BlendMode::Color => blend::color(&mut base_rgb, &blend_rgb),
        BlendMode::ColorBurn => blend::color_burn(&mut base_rgb, &blend_rgb),
        BlendMode::ColorDodge => blend::color_dodge(&mut base_rgb, &blend_rgb),
        BlendMode::Darken => blend::darken(&mut base_rgb, &blend_rgb),
        BlendMode::Difference => blend::difference(&mut base_rgb, &blend_rgb),
        BlendMode::Divide => blend::divide(&mut base_rgb, &blend_rgb),
        BlendMode::DestinationIn => blend::destination_in(&mut base_rgba, &blend_rgba, opacity),
        BlendMode::DestinationOut => blend::destination_out(&mut base_rgba, &blend_rgba, opacity),
        BlendMode::Exclusion => blend::exclusion(&mut base_rgb, &blend_rgb),
        BlendMode::HardLight => blend::hard_light(&mut base_rgb, &blend_rgb),
        BlendMode::Hue => blend::hue(&mut base_rgb, &blend_rgb),
        BlendMode::Lighten => blend::lighten(&mut base_rgb, &blend_rgb),
        BlendMode::Luminosity => blend::luminosity(&mut base_rgb, &blend_rgb),
        BlendMode::Multiply => blend::multiply(&mut base_rgb, &blend_rgb),
        // Pass through isn’t valid because it is only for groups, but
        // we’re just going to treat it like normal blending for now.
        BlendMode::Normal | BlendMode::PassThrough => base_rgb = blend_rgb,
        BlendMode::Overlay => blend::overlay(&mut base_rgb, &blend_rgb),
        BlendMode::Saturation => blend::saturation(&mut base_rgb, &blend_rgb),
        BlendMode::Screen => blend::screen(&mut base_rgb, &blend_rgb),
        BlendMode::SoftLight => blend::soft_light(&mut base_rgb, &blend_rgb),
        BlendMode::Subtract => blend::subtract(&mut base_rgb, &blend_rgb),
        BlendMode::Replace => {
            let alpha = (opacity * blend_color.alpha as f32).round() as u8;
            color.red = blend_color.red;
            color.green = blend_color.green;
            color.blue = blend_color.blue;
            color.alpha = alpha;
            return;
        }
    }

    let mut output: RgbaColor;

    if blend_mode.is_porter_duff() || blend_mode == BlendMode::Replace {
        output = base_rgba;
    } else {
        // Very useful documentation on this at https://drafts.fxtf.org/compositing-1
        // Cs = (1 - αb) x Cs + αb x B(Cb, Cs)
        // Co = αs x Fa x Cs + αb x Fb x Cb
        // Fa = 1; Fb = 1 – αs
        let blend_alpha = opacity * blend_rgba.alpha;
        let base_alpha = base_rgba.alpha;

        // Ignore the alpha for the following calculations.
        blend_rgba.alpha = 1.0;
        base_rgba.alpha = 1.0;

        // co = Cs x αs + Cb x αb x (1 - αs)
        output = base_rgb.into();
        output = blend_rgba * (1.0 - base_alpha) + output * base_alpha;
        output = output * blend_alpha + base_rgba * (base_alpha * (1.0 - blend_alpha));

        // TODO: Pass in the premultiply flag and only unpremultiply if should_premultiply is false.
        output.unpremultiply();
    }

    let result = output.to_color();

    color.red = result.red;
    color.green = result.green;
    color.blue = result.blue;
    color.alpha = result.alpha;
}

/// Options for compositing.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CompositeOperationOptions<'a> {
    /// The blend mode.
    pub blend_mode: BlendMode,
    /// The masks.
    pub masks: Vec<Mask<'a>>,
}

#[cfg(test)]
mod test {
    use std::borrow::Cow;

    use crate::{Mask, Size, TiledMask};

    use super::*;

    #[test]
    fn test_blend_colors_with_top_opacity() {
        let mut color = Color::from_rgb_u32(0xffffff);

        let mut blend_color = Color::from_rgb_u32(0x0000ff);
        blend_color.alpha = 128;

        blend_colors(&mut color, &blend_color, BlendMode::Normal, 1.0);

        assert_eq!(color.red, 0x7f, "Reds don’t match.");
        assert_eq!(color.green, 0x7f, "Greens don’t match.");
        assert_eq!(color.blue, 0xff, "Blues don’t match.");
        assert_eq!(color.alpha, 0xff, "Alphas don’t match.");
    }

    #[test]
    fn test_blend_colors_with_opacity() {
        let mut color = Color::from_rgb_u32(0xffffff);
        color.alpha = 51;

        let mut blend_color = Color::from_rgb_u32(0x0000ff);
        blend_color.alpha = 128;

        blend_colors(&mut color, &blend_color, BlendMode::Normal, 1.0);

        assert_eq!(color.red, 0x2a, "Reds don’t match.");
        assert_eq!(color.green, 0x2a, "Greens don’t match.");
        assert_eq!(color.blue, 0xff, "Blues don’t match.");
        assert_eq!(color.alpha, 153, "Alphas don’t match.");
    }

    #[test]
    fn draw_layer_with_tiled_mask() {
        let mut base_image = Image::color(
            &Color::from_rgb_u32(0x639bff),
            Size {
                width: 16,
                height: 8,
            },
        );
        let image = Image::color(
            &Color::from_rgb_u32(0xcbdbfc),
            Size {
                width: 13,
                height: 6,
            },
        );
        let position = Point { x: 1.0, y: 1.0 };
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

        draw_layer_over_image(&mut base_image, &layer);

        // Reposition the layer and draw again to make sure
        // that the mask stays put.
        layer.position.x += 1.0;
        draw_layer_over_image(&mut base_image, &layer);

        base_image.save("/tmp/tiled_mask.png").unwrap();

        let expected_image = Image::open("tests/images/tiled_mask.png").unwrap();

        assert!(base_image.appears_equal_to(&expected_image));
    }

    #[test]
    fn draw_layer_with_tiled_mask_negative_position() {
        let mut base_image = Image::color(
            &Color::from_rgb_u32(0x639bff),
            Size {
                width: 16,
                height: 8,
            },
        );
        let image = Image::color(
            &Color::from_rgb_u32(0xcbdbfc),
            Size {
                width: 13,
                height: 6,
            },
        );
        let position = Point { x: -1.0, y: -1.0 };
        let mut layer = Layer::new(&image, position);

        // Setting up a checkerboard image.
        let mut image = Image::empty(Size {
            width: 3,
            height: 3,
        });
        image.set_pixel_color(Color::BLACK, Point::zero());
        image.set_pixel_color(Color::BLACK, Point { x: 1, y: 1 });

        let mask = TiledMask {
            image: Cow::Borrowed(&image),
            offset: Point { x: 0, y: 0 },
        };
        let mask = Mask::Tiled(mask);
        layer.masks = vec![mask];

        draw_layer_over_image(&mut base_image, &layer);

        layer.position.x -= 1.0;
        draw_layer_over_image(&mut base_image, &layer);

        base_image.save("/tmp/tiled_mask_negative.png").unwrap();

        let expected_image = Image::open("tests/images/tiled_mask_negative.png").unwrap();

        assert!(base_image.appears_equal_to(&expected_image));
    }

    #[test]
    #[ignore]
    fn draw_layer_performance() {
        let size = Size {
            width: 10,
            height: 10,
        };
        // let size = Size {
        //     width: 4,
        //     height: 4,
        // };
        let mut base_image = Image::color(&Color::from_rgb_u32(0x639bff), size);
        let mut image = Image::color(&Color::from_rgb_u32(0xcbdbfc), size);
        image.set_pixel_color(Color::CLEAR, Point { x: 2, y: 1 });
        image.set_pixel_color(Color::GREEN, Point { x: 0, y: 0 });
        image.set_pixel_color(Color::CYAN, Point { x: 1, y: 0 });
        // image.save("/tmp/loco.png").unwrap();
        let position = Point { x: 0.0, y: -1.0 };
        let layer = Layer::new(&image, position);

        let now = std::time::Instant::now();
        draw_layer_over_image(&mut base_image, &layer);
        // 670ms
        println!("🍟 time taken: {:?}", now.elapsed());

        base_image.save("/tmp/base_image.png").unwrap();
        panic!();
    }
}
