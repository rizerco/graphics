use std::cmp::min;

use crate::{BlendMode, Color, Image};

use super::{
    blend::{self, RgbaColor},
    operation::Operation,
    Layer,
};

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
    let end_x = layer.image.size.width as i32 + location.x;
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
    let end_y = layer.image.size.height as i32 + location.y;
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

    let y_offset = if location.y < 0 {
        location.y.abs() as u32
    } else {
        0
    };

    // I tried using rayon for this, but with 10,000 rows the performance
    // was a little worse with rayon than without.
    for y in 0..required_height {
        let offset = ((y + y_offset) * layer.image.bytes_per_row) as usize; //+ y_offset;
        let target_offset = ((target_y_offset + y) * image.bytes_per_row) as i32;
        let target_offset = (target_offset + (start_x as i32) * 4) as usize;
        // Using a second loop was a tiny bit faster than splicing the vec.
        for x in (0..required_width * 4).step_by(4) {
            let start = offset + x + x_offset;
            let data = layer.image.data.get(start..(start + 4)).unwrap();
            let blend_color: [u8; 4] = data.try_into().unwrap();
            let blend_color: Color = blend_color.into();

            let start = target_offset + x;
            let data = image.data.get(start..(start + 4)).unwrap();
            let base_color: [u8; 4] = data.try_into().unwrap();
            let mut base_color: Color = base_color.into();

            blend_colors(
                &mut base_color,
                &blend_color,
                layer.blend_mode,
                layer.opacity,
            );
            // let base_color = Color::RED;

            image.data[target_offset + x + 0] = base_color.red;
            image.data[target_offset + x + 1] = base_color.green;
            image.data[target_offset + x + 2] = base_color.blue;
            image.data[target_offset + x + 3] = base_color.alpha;
        }
    }
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
    }

    let mut output: RgbaColor;

    if blend_mode.is_porter_duff() {
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

#[cfg(test)]
mod test {
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
}
