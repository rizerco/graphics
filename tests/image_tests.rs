#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use graphics::{Color, Image, Point, Rect, Size};
    use image::ImageFormat;

    #[test]
    fn test_file_data() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar.png");
        let image = Image::open(path).unwrap();

        let png_data = image.file_data(ImageFormat::Png).unwrap();

        let image_from_file = Image::from_file_data(png_data.as_slice()).unwrap();

        assert!(image.appears_equal_to(&image_from_file));
    }

    #[test]
    fn test_trim() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar-transparent.png");
        let mut image = Image::open(path).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar-trimmed.png");
        let expected_image = Image::open(path).unwrap();

        let trimmed_rect = image.trim().unwrap();

        // image.save("/tmp/yknow.png").unwrap();

        assert!(image.appears_equal_to(&expected_image));
        assert_eq!(trimmed_rect, Rect::new(4, 4, 12, 13));
    }

    #[test]
    fn test_trim_when_not_required() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar.png");
        let mut image = Image::open(path).unwrap();

        let expected_image = image.clone();

        let trimmed_rect = image.trim().unwrap();

        assert!(image.appears_equal_to(&expected_image));
        assert_eq!(trimmed_rect, Rect::new(0, 0, 20, 21));
    }

    #[test]
    fn test_draw_image_over() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar.png");
        let mut image = Image::open(path).unwrap();

        let color_image = Image::color(
            &Color::from_rgb_u32(0x00eba6),
            Size {
                width: 12,
                height: 7,
            },
        );

        let location = Point { x: 3, y: 4 };
        image.draw_image_over(&color_image, location);
        // image.save("/tmp/drawn-over(3,4).png").unwrap();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/drawn-over(3,4).png");
        let expected_image = Image::open(path).unwrap();
        assert!(image.appears_equal_to(&expected_image));
    }

    #[test]
    fn test_draw_image_over_off_edge() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar.png");
        let original_image = Image::open(path).unwrap();

        let mut color_image = Image::color(
            &Color::from_rgb_u32(0x00eba6),
            Size {
                width: 12,
                height: 7,
            },
        );
        // Set the colour of the second pixel to yellow.
        color_image.data[4] = 0xd6;
        color_image.data[6] = 0x00;

        // Test when the image goes off the canvas on the bottom right.
        let mut image = original_image.clone();
        let location = Point { x: 15, y: 18 };
        image.draw_image_over(&color_image, location);
        image.save("/tmp/drawn-over(15,18).png").unwrap();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/drawn-over(15,18).png");
        let expected_image = Image::open(path).unwrap();
        assert!(image.appears_equal_to(&expected_image));

        // Test when the image goes off the canvas on the top left.
        let mut image = original_image.clone();
        let location = Point { x: -3, y: -2 };
        image.draw_image_over(&color_image, location);
        image.save("/tmp/drawn-over(-3,-2).png").unwrap();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/drawn-over(-3,-2).png");
        let expected_image = Image::open(path).unwrap();
        assert!(image.appears_equal_to(&expected_image));
    }

    #[test]
    fn test_draw_image_over_out_of_bounds() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar.png");
        let original_image = Image::open(path).unwrap();

        let color_image = Image::color(
            &Color::from_rgb_u32(0x00eba6),
            Size {
                width: 12,
                height: 7,
            },
        );

        // Test when the location is too far left.
        let mut image = original_image.clone();
        let location = Point { x: -12, y: 4 };
        image.draw_image_over(&color_image, location);
        assert!(image.appears_equal_to(&original_image));

        // Test when the location is too far right.
        let mut image = original_image.clone();
        let location = Point { x: 20, y: 4 };
        image.draw_image_over(&color_image, location);
        assert!(image.appears_equal_to(&original_image));

        // Test when the location is too hight.
        let mut image = original_image.clone();
        let location = Point { x: 3, y: -7 };
        image.draw_image_over(&color_image, location);
        assert!(image.appears_equal_to(&original_image));

        // Test when the location is too low.
        let mut image = original_image.clone();
        let location = Point { x: 3, y: 21 };
        image.draw_image_over(&color_image, location);
        assert!(image.appears_equal_to(&original_image));
    }

    #[test]
    fn test_flip_horizontally() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar.png");
        let mut image = Image::open(path).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar-flipped-horizontally.png");
        let expected_image = Image::open(path).unwrap();

        image.flip_horizontally();

        // image.save("/tmp/avatar-flipped-horizontally.png").unwrap();
        assert!(image.appears_equal_to(&expected_image));
    }

    #[test]
    fn test_flip_vertically() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar.png");
        let mut image = Image::open(path).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar-flipped-vertically.png");
        let expected_image = Image::open(path).unwrap();

        image.flip_vertically();

        image.save("/tmp/avatar-flipped-vertically.png").unwrap();
        assert!(image.appears_equal_to(&expected_image));
    }

    #[test]
    fn scaled_up() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar.png");
        let image = Image::open(path).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar-scaled-3x.png");
        let expected_image = Image::open(path).unwrap();

        let image = image.scaled_up(3);

        image.save("/tmp/avatar-scaled-3x.png").unwrap();
        assert!(image.appears_equal_to(&expected_image));
    }

    #[test]
    fn test_resized() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar.png");
        let mut image = Image::open(path).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/avatar-resized.png");
        let expected_image = Image::open(path).unwrap();

        image.resize_nearest_neighbor(expected_image.size);

        image.save("/tmp/avatar-resized.png").unwrap();
        assert!(image.appears_equal_to(&expected_image));
    }

    #[test]
    fn test_rotated() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/tv.png");
        let mut image = Image::open(path).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/tv-rotated-nn.png");
        let expected_image = Image::open(path).unwrap();

        let midpoint = Point {
            x: image.size.width as f32 * 0.5,
            y: image.size.height as f32 * 0.5,
        };
        image.rotate_nearest_neighbor(std::f32::consts::PI * 0.2, midpoint);

        image.save("/tmp/tv-rotated-nn.png").unwrap();
        assert!(image.appears_equal_to(&expected_image));
    }

    #[test]
    fn test_2x2_rotated() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/2x2.png");
        let mut image = Image::open(path).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/2x2-rotated.png");
        let expected_image = Image::open(path).unwrap();

        let midpoint = Point {
            x: image.size.width as f32 * 0.5,
            y: image.size.height as f32 * 0.5,
        };
        image.rotate_nearest_neighbor(std::f32::consts::PI * 0.5, midpoint);

        image.save("/tmp/2x2-rotated.png").unwrap();
        assert!(image.appears_equal_to(&expected_image));
    }

    #[test]
    fn test_3x2_rotated() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/3x2.png");
        let mut image = Image::open(path).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/3x2-rotated.png");
        let expected_image = Image::open(path).unwrap();

        let midpoint = Point {
            x: image.size.width as f32 * 0.5,
            y: image.size.height as f32 * 0.5,
        };
        let midpoint = midpoint.floored().into();
        image.rotate_nearest_neighbor(std::f32::consts::PI * 0.5, midpoint);

        image.save("/tmp/3x2-rotated.png").unwrap();
        assert!(image.appears_equal_to(&expected_image));
    }
}
