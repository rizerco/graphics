use std::cmp;

use crate::{
    composite::{self, Layer},
    BlendMode, Color, Image, Mask, Point, Rect,
};

/// Replaces all instances of one colour with another.
pub fn replace_color(image: &mut Image, target_color: &Color, replacement_color: &Color) {
    let target_color: [u8; 4] = target_color.into();
    for y in 0..image.size.height {
        let offset = (y * image.bytes_per_row) as usize;
        for x in (0..image.size.width * 4).step_by(4) {
            let start = offset + x as usize;
            let data = image.data.get(start..(start + 4)).unwrap();

            if data == target_color {
                image.data[start + 0] = replacement_color.red;
                image.data[start + 1] = replacement_color.green;
                image.data[start + 2] = replacement_color.blue;
                image.data[start + 3] = replacement_color.alpha;
            }
        }
    }
}

/// Returns an image the same size as the source image
/// where any corresponding pixels of the target colour
/// in the source image are output as black, and all other
/// pixels are transparent.
pub fn mask_image(source_image: &Image, target_color: &Color) -> Image {
    let mut image = Image::empty(source_image.size);
    for y in 0..image.size.height {
        let offset = (y * image.bytes_per_row) as usize;
        for x in 0..image.size.width {
            let start = offset + (x * 4) as usize;
            let point = Point { x, y };

            if source_image
                .pixel_color(point.into())
                .is_some_and(|color| &color == target_color)
            {
                image.data[start + 0] = 0;
                image.data[start + 1] = 0;
                image.data[start + 2] = 0;
                image.data[start + 3] = u8::MAX;
            }
        }
    }
    image
}

/// Performs a flood fill on an image within a bounding box.
/// Returns the area affected by the flood fill.
/// If the `secondary_image` is supplied, this will also
/// be recocoloured, but not referenced when computing the
/// area to be filled.
fn flood_fill_in_bounds(
    image: &mut Image,
    start: Point<i32>,
    fill_color: &Color,
    secondary_image: Option<&mut Image>,
    bounding_box: Option<Rect<i32>>,
) -> anyhow::Result<Rect<i32>> {
    let image_bounds = Rect {
        origin: Point::zero(),
        size: image.size.into(),
    };
    let bounding_box = bounding_box.unwrap_or(image_bounds);

    // A selection outside of the bounds of the image is possible, so clamp
    // the bounding box to the image bounds.
    let bounding_box = bounding_box
        .intersection(&image_bounds)
        .ok_or(anyhow::anyhow!("Bounding box is outside of the image."))?;

    // Check that the point is actually inside the bounds.
    if !bounding_box.contains(start) {
        anyhow::bail!("Point outside of image bounds.");
    }

    let min_x = bounding_box.min_x();
    let max_x = bounding_box.max_x();
    let min_y = bounding_box.min_y();
    let max_y = bounding_box.max_y();

    let vertex_buffer = &mut image.data;
    let bytes_per_row = image.bytes_per_row;

    // This is pretty horrible, but the combination of an optional
    // mutable borrow, plus the loops is making the borrow checker
    // put up a fight.
    let has_secondary_image = secondary_image.is_some();
    let mut some = Vec::new();
    let secondary_vertex_buffer = if let Some(secondary_image) = secondary_image {
        if secondary_image.size != image.size
            || secondary_image.bytes_per_row != image.bytes_per_row
        {
            anyhow::bail!("The secondary image’s properties do not match the primary’s.")
        }
        &mut secondary_image.data
    } else {
        &mut some
    };

    let mut affected_min_x = start.x;
    let mut affected_max_x = start.x;
    let mut affected_min_y = start.y;
    let mut affected_max_y = start.y;

    // Algorithm is based off http://lodev.org/cgtutor/floodfill.html
    // Scanline Floodfill Algorithm With Stack.
    // Target colour is the colour we want to replace.
    let target_color = unsigned_int_color(start, vertex_buffer, bytes_per_row);
    let new_color = fill_color.as_rgba_u32();

    let mut points: Vec<Point<i32>> = Vec::new();
    points.push(start);

    let mut color: u32;
    let mut span_left;
    let mut span_right;

    while !points.is_empty() {
        let Some(mut current_point) = points.pop() else {
            continue;
        };
        color = unsigned_int_color(current_point, vertex_buffer, bytes_per_row);

        while current_point.y >= min_y && color == target_color {
            current_point.y -= 1;

            if current_point.y >= min_y {
                color = unsigned_int_color(current_point, vertex_buffer, bytes_per_row);
            }
        }

        current_point.y += 1;

        span_left = false;
        span_right = false;

        color = unsigned_int_color(current_point, vertex_buffer, bytes_per_row);

        while current_point.y < max_y && color == target_color && new_color != color {
            // Change the old colour to the new colour’s RGBA value.
            let byte_index =
                bytes_per_row as usize * current_point.y as usize + current_point.x as usize * 4;

            vertex_buffer[byte_index + 0] = ((0xff000000 & new_color) >> 24) as u8;
            vertex_buffer[byte_index + 1] = ((0x00ff0000 & new_color) >> 16) as u8;
            vertex_buffer[byte_index + 2] = ((0x0000ff00 & new_color) >> 8) as u8;
            vertex_buffer[byte_index + 3] = (0x000000ff & new_color) as u8;

            if has_secondary_image {
                secondary_vertex_buffer[byte_index + 0] = ((0xff000000 & new_color) >> 24) as u8;
                secondary_vertex_buffer[byte_index + 1] = ((0x00ff0000 & new_color) >> 16) as u8;
                secondary_vertex_buffer[byte_index + 2] = ((0x0000ff00 & new_color) >> 8) as u8;
                secondary_vertex_buffer[byte_index + 3] = (0x000000ff & new_color) as u8;
            }

            if current_point.x > min_x {
                let west_point = Point {
                    x: current_point.x - 1,
                    y: current_point.y,
                };

                color = unsigned_int_color(west_point, &vertex_buffer, bytes_per_row);

                if !span_left && color == target_color {
                    points.push(west_point);
                    span_left = true;
                } else if span_left && color != target_color {
                    span_left = false;
                }
            }

            if current_point.x < (max_x - 1) {
                let east_point = Point {
                    x: current_point.x + 1,
                    y: current_point.y,
                };

                color = unsigned_int_color(east_point, &vertex_buffer, bytes_per_row);

                if !span_right && color == target_color {
                    points.push(east_point);
                    span_right = true;
                } else if span_right && color != target_color {
                    span_right = false;
                }
            }

            if !span_right || !span_left {
                affected_min_x = cmp::min(affected_min_x, current_point.x);
                affected_max_x = cmp::max(affected_max_x, current_point.x);
                affected_min_y = cmp::min(affected_min_y, current_point.y);
                affected_max_y = cmp::max(affected_max_y, current_point.y);
            }

            current_point.y += 1;

            if current_point.y < max_y {
                color = unsigned_int_color(current_point, &vertex_buffer, bytes_per_row);
            }
        }
    }

    let affected_region = Rect::new(
        affected_min_x,
        affected_min_y,
        affected_max_x - affected_min_x + 1,
        affected_max_y - affected_min_y + 1,
    );

    Ok(affected_region)
}

/// Fills the selected colour from the starting point to all
/// all pixels the same colour as the starting point.
pub fn flood_fill(
    image: &mut Image,
    start: Point<i32>,
    fill_color: &Color,
) -> anyhow::Result<Rect<i32>> {
    flood_fill_in_bounds(image, start, fill_color, None, None)
}

/// Fills the selected colour from the starting point to all
/// all pixels the same colour as the starting point within
/// a masked region.
pub fn flood_fill_with_mask(
    image: &mut Image,
    start: Point<i32>,
    fill_color: &Color,
    mask: &dyn Mask,
) -> anyhow::Result<Rect<i32>> {
    let bounding_box = Some(mask.bounding_box());
    let mut result = image.clone();
    let affected_region = flood_fill_in_bounds(&mut result, start, fill_color, None, bounding_box)?;
    if fill_color.alpha == 0 {
        // For a clear, erase the masked area,
        // then just draw the two images on top of each other.
        let mut layer = Layer::new(&mask.image(), mask.bounding_box().origin.into());
        layer.blend_mode = BlendMode::DestinationOut;
        let mut image_with_mask_erased = image.clone();
        composite::draw_layer_over_image(&mut image_with_mask_erased, &layer);
        let layer = Layer::new(&image_with_mask_erased, Point::zero());
        composite::draw_layer_over_image(&mut result, &layer);
        *image = result;
    } else {
        let subimage = result.subimage_masked(mask)?;
        let layer = Layer::new(&subimage, mask.bounding_box().origin.into());
        composite::draw_layer_over_image(image, &layer);
    }
    Ok(affected_region)
}

/// Performs a flood fill referencing one image but
/// recolouring another.
pub fn flood_fill_with_reference(
    target_image: &mut Image,
    reference_image: &Image,
    start: Point<i32>,
    fill_color: &Color,
) -> anyhow::Result<Rect<i32>> {
    let mut reference_clone = reference_image.clone();
    let affected_region = flood_fill_in_bounds(
        &mut reference_clone,
        start,
        fill_color,
        Some(target_image),
        None,
    )?;
    Ok(affected_region)
}

// MARK: Helper methods

/// Helper method for the bucket fill that returns an array for the colour at a point.
fn unsigned_int_color(point: Point<i32>, vertex_buffer: &Vec<u8>, bytes_per_row: u32) -> u32 {
    let offset = bytes_per_row as usize * point.y as usize + point.x as usize * 4;

    if vertex_buffer.len() < offset + 4 {
        return 0;
    }

    let first_channel = vertex_buffer[offset + 0] as u32;
    let second_channel = vertex_buffer[offset + 1] as u32;
    let third_channel = vertex_buffer[offset + 2] as u32;
    let fourth_channel = vertex_buffer[offset + 3] as u32;

    first_channel << 24 | second_channel << 16 | third_channel << 8 | fourth_channel
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::Size;

    use super::*;

    #[test]
    fn test_mask_image() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar.png");
        let image = Image::open(path).unwrap();
        let target_color = Color::from_rgb_u32(0xe8b796);
        let result = mask_image(&image, &target_color);

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/mask-image.png");
        let expected_image = Image::open(path).unwrap();

        result.save("/tmp/mask-image.png").unwrap();

        assert!(result.appears_equal_to(&expected_image));
    }

    #[test]
    fn test_flood_fill() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/red_32.png");
        let mut image = Image::open(path).unwrap();
        let fill_color = Color::from_rgb_u32(0x00ffff);
        let start = Point { x: 2, y: 5 };
        let result = flood_fill(&mut image, start, &fill_color).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/cyan_32.png");
        let expected_image = Image::open(path).unwrap();

        assert!(image.appears_equal_to(&expected_image));

        assert_eq!(result.origin, Point::zero());
        assert_eq!(
            result.size,
            Size {
                width: 32,
                height: 32
            }
        );
    }

    #[test]
    fn test_flood_fill_with_avatar() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar.png");
        let mut image = Image::open(path).unwrap();
        let fill_color = Color::from_rgb_u32(0xde0154);
        let start = Point { x: 9, y: 7 };
        let result = flood_fill(&mut image, start, &fill_color).unwrap();

        // image.save("/tmp/*result.png").unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar-flood-filled.png");
        let expected_image = Image::open(path).unwrap();

        assert!(image.appears_equal_to(&expected_image));

        assert_eq!(result.origin, Point { x: 5, y: 5 });
        assert_eq!(
            result.size,
            Size {
                width: 10,
                height: 5
            }
        );
    }

    #[test]
    fn test_flood_fill_with_reference() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar.png");
        let reference_image = Image::open(path).unwrap();
        let mut output_image = Image::empty(reference_image.size);
        let fill_color = Color::from_rgb_u32(0xde0154);
        let start = Point { x: 9, y: 7 };
        let result =
            flood_fill_with_reference(&mut output_image, &reference_image, start, &fill_color)
                .unwrap();

        output_image.save("/tmp/*result.png").unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar-reference-fill.png");
        let expected_image = Image::open(path).unwrap();

        assert!(output_image.appears_equal_to(&expected_image));

        assert_eq!(result.origin, Point { x: 5, y: 5 });
        assert_eq!(
            result.size,
            Size {
                width: 10,
                height: 5
            }
        );
    }

    struct TestMask {
        image: Image,
        bounding_box: Rect<i32>,
    }

    impl Mask for TestMask {
        fn image(&self) -> &Image {
            &self.image
        }

        fn bounding_box(&self) -> Rect<i32> {
            self.bounding_box
        }
    }

    #[test]
    fn test_flood_fill_with_mask() {
        let manifest_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut path = manifest_directory.clone();
        path.push("tests/images/map.png");
        let image = Image::open(path).unwrap();

        let mut path = manifest_directory.clone();
        path.push("tests/images/mask.png");
        let mask_image = Image::open(path).unwrap();

        let mask = TestMask {
            image: mask_image,
            bounding_box: Rect::new(6, 14, 15, 15),
        };

        let fill_color = Color::from_rgb_u32(0x70AEBF);
        let mut result_01 = image.clone();
        let mut result_02 = image.clone();

        let mut path = manifest_directory.clone();
        path.push("tests/images/map_filled_mask_01.png");
        let expected_image_01 = Image::open(path).unwrap();

        flood_fill_with_mask(&mut result_01, Point { x: 12, y: 19 }, &fill_color, &mask).unwrap();

        assert!(result_01.appears_equal_to(&expected_image_01));

        let mut path = manifest_directory.clone();
        path.push("tests/images/map_filled_mask_02.png");
        let expected_image_02 = Image::open(path).unwrap();

        flood_fill_with_mask(&mut result_02, Point { x: 15, y: 25 }, &fill_color, &mask).unwrap();

        assert!(result_02.appears_equal_to(&expected_image_02));
    }

    #[test]
    fn test_flood_fill_erase_with_mask() {
        let manifest_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let mut path = manifest_directory.clone();
        path.push("tests/images/map.png");
        let image = Image::open(path).unwrap();

        let mut path = manifest_directory.clone();
        path.push("tests/images/mask.png");
        let mask_image = Image::open(path).unwrap();

        let mask = TestMask {
            image: mask_image,
            bounding_box: Rect::new(6, 14, 15, 15),
        };

        let fill_color = Color::CLEAR;
        let mut result_01 = image.clone();
        let mut result_02 = image.clone();

        let mut path = manifest_directory.clone();
        path.push("tests/images/map_filled_mask_01_erase.png");
        let expected_image_01 = Image::open(path).unwrap();

        flood_fill_with_mask(&mut result_01, Point { x: 12, y: 19 }, &fill_color, &mask).unwrap();

        assert!(result_01.appears_equal_to(&expected_image_01));

        let mut path = manifest_directory.clone();
        path.push("tests/images/map_filled_mask_02_erase.png");
        let expected_image_02 = Image::open(path).unwrap();

        flood_fill_with_mask(&mut result_02, Point { x: 15, y: 25 }, &fill_color, &mask).unwrap();

        assert!(result_02.appears_equal_to(&expected_image_02));
    }

    #[test]
    fn test_replace_color() {
        let bytes = vec![
            0xff, 0xba, 0x43, 0xff, // 1
            0x54, 0x00, 0x13, 0xff, // 2
            0xda, 0xda, 0x01, 0xff, // 3
            0x54, 0x00, 0x13, 0x7a, // 4
        ];
        let mut image = Image::new(
            bytes,
            Size {
                width: 2,
                height: 2,
            },
            8,
        );

        let target_color = Color::from_rgb_u32(0x540013);
        let replacement_color = Color::from_rgb_u32(0xff13ff);
        replace_color(&mut image, &target_color, &replacement_color);

        let expected_bytes = vec![
            0xff, 0xba, 0x43, 0xff, // 1
            0xff, 0x13, 0xff, 0xff, // 2
            0xda, 0xda, 0x01, 0xff, // 3
            0x54, 0x00, 0x13, 0x7a, // 4
        ];

        assert_eq!(image.data, expected_bytes);
    }
}
