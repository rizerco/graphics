use std::path::PathBuf;

use graphics::{
    composite::{Layer, Operation},
    *,
};

#[test]
fn addition_compositing() {
    let blend_mode = BlendMode::Addition;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn color_compositing() {
    let blend_mode = BlendMode::Color;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn color_compositing_50() {
    let blend_mode = BlendMode::Color;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn color_burn_compositing() {
    let blend_mode = BlendMode::ColorBurn;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn color_dodge_compositing() {
    let blend_mode = BlendMode::ColorDodge;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn darken_compositing() {
    let blend_mode = BlendMode::Darken;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn darken_compositing_50() {
    let blend_mode = BlendMode::Darken;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn destination_in_50() {
    let blend_mode = BlendMode::DestinationIn;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn destination_out_50() {
    let blend_mode = BlendMode::DestinationOut;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn difference_compositing() {
    let blend_mode = BlendMode::Difference;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn difference_compositing_50() {
    let blend_mode = BlendMode::Difference;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn divide_compositing() {
    let blend_mode = BlendMode::Divide;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn divide_compositing_50() {
    let blend_mode = BlendMode::Divide;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn exclusion_compositing() {
    let blend_mode = BlendMode::Exclusion;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn exclusion_compositing_50() {
    let blend_mode = BlendMode::Exclusion;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn hard_light_compositing() {
    let blend_mode = BlendMode::HardLight;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn hard_light_compositing_50() {
    let blend_mode = BlendMode::HardLight;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn hue_compositing() {
    let blend_mode = BlendMode::Hue;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn hue_compositing_50() {
    let blend_mode = BlendMode::Hue;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn lighten_compositing() {
    let blend_mode = BlendMode::Lighten;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn lighten_compositing_50() {
    let blend_mode = BlendMode::Lighten;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn luminosity_compositing() {
    let blend_mode = BlendMode::Luminosity;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn luminosity_compositing_50() {
    let blend_mode = BlendMode::Luminosity;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn multiply_compositing() {
    let blend_mode = BlendMode::Multiply;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn multiply_compositing_50() {
    let blend_mode = BlendMode::Multiply;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn normal_compositing() {
    let blend_mode = BlendMode::Normal;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn normal_compositing_50() {
    let blend_mode = BlendMode::Normal;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn overlay_compositing() {
    let blend_mode = BlendMode::Overlay;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn overlay_compositing_50() {
    let blend_mode = BlendMode::Overlay;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn saturation_compositing() {
    let blend_mode = BlendMode::Saturation;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn saturation_compositing_50() {
    let blend_mode = BlendMode::Saturation;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn screen_compositing() {
    let blend_mode = BlendMode::Screen;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn screen_compositing_50() {
    let blend_mode = BlendMode::Screen;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn soft_light_compositing() {
    let blend_mode = BlendMode::SoftLight;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn soft_light_compositing_50() {
    let blend_mode = BlendMode::SoftLight;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn subtract_compositing() {
    let blend_mode = BlendMode::Subtract;
    let opacity = 1.0;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn subtract_compositing_50() {
    let blend_mode = BlendMode::Subtract;
    let opacity = 0.5;
    run_blend_mode_test(blend_mode, opacity);
}

#[test]
fn offset_normal() {
    let blend_mode = BlendMode::Normal;
    run_offset_blend_mode_test(blend_mode, 1.0);
}

#[test]
fn offset_normal_50() {
    let blend_mode = BlendMode::Normal;
    run_offset_blend_mode_test(blend_mode, 0.5);
}

#[test]
fn offset_hue() {
    let blend_mode = BlendMode::Hue;
    run_offset_blend_mode_test(blend_mode, 1.0);
}

#[test]
fn offset_hue_50() {
    let blend_mode = BlendMode::Hue;
    run_offset_blend_mode_test(blend_mode, 0.5);
}

#[test]
fn base_and_blend_opacity_50() {
    let base_position = Point { x: 20.0, y: 10.0 };
    let blend_position = Point { x: -20.0, y: 40.0 };
    let blend_mode = BlendMode::Normal;
    let output_name = "mountain-base-50-blend-50".to_string();
    run_blend_mode_test_with_position(
        blend_mode,
        base_position,
        blend_position,
        0.5,
        0.5,
        output_name,
        None,
    );
}

#[test]
fn background_with_opacity() {
    let output_name = "small-color-burn-with-alpha";

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/images/small-with-alpha.png");
    let image = Image::open(path).unwrap();

    let color_image = Image::color(&Color::from_rgb_u32(0x99e550), image.size);

    let size = image.size;
    let image_layer = Layer::new(image, Point::zero());
    let mut color_layer = Layer::new(color_image, Point::zero());
    color_layer.blend_mode = BlendMode::ColorBurn;

    let layers = vec![image_layer, color_layer];

    let operation = Operation::new(layers, size);
    let result = composite(&operation);

    result
        .save(format!("/tmp/{}.png", output_name).as_str())
        .unwrap();

    let directory = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let file_path = format!("{}/tests/images/{}.png", directory, output_name);
    println!("{}", file_path);

    let expected_image = Image::open(file_path).unwrap();

    let images_are_equal = result.appears_equal_to(&expected_image);
    assert!(images_are_equal);
}

// #[test]
// fn container_size() {
//     let base_position = Point { x: -20.0, y: -10.0 };
//     let blend_position = Point { x: -20.0, y: 10.0 };
//     let blend_mode = BlendMode::Normal;
//     let output_name = "mountain-cropped".to_string();
//     let size = Size {
//         width: 30,
//         height: 27,
//     };
//     run_blend_mode_test_with_position(
//         blend_mode,
//         base_position,
//         blend_position,
//         1.0,
//         0.7,
//         output_name,
//         Some(size),
//     );
// }

#[test]
fn single_semitransparent_image() {
    let size = Size {
        width: 1,
        height: 1,
    };
    let color = Color::from_rgb_u32(0x5fcde4);
    let image = Image::color(&color, size);
    let mut layer = Layer::new(image, Point::zero());
    layer.opacity = 0.5;

    let expected_bytes = vec![0x5f, 0xcd, 0xe4, 0x80];

    let operation = Operation::new(vec![layer], size);

    let result = composite(&operation);
    assert_eq!(result.data, expected_bytes);
}

#[test]
fn transparent_base_with_semitransparent_blend() {
    let size = Size {
        width: 1,
        height: 1,
    };
    let base_image = Image::empty(size);
    let base_layer = Layer::new(base_image, Point::zero());
    let color = Color::from_rgb_u32(0x5fcde4);
    let image = Image::color(&color, size);
    let mut blend_layer = Layer::new(image, Point::zero());
    blend_layer.opacity = 0.5;

    let expected_bytes = vec![0x5f, 0xcd, 0xe4, 0x80];

    let operation = Operation::new(vec![base_layer, blend_layer], size);

    let result = composite(&operation);
    assert_eq!(result.data, expected_bytes);
}

// #[test]
// fn resize() {
//     let background = io::load_image(include_bytes!("images/mountain.png"));
//     let mut foreground = io::load_image(include_bytes!("images/gerbil.jpg"));
//     foreground.position.x = 15.4;
//     foreground.position.y = 13.3;
//     foreground.size_on_canvas.width = 128.0;
//     foreground.size_on_canvas.height = 87.0;

//     let operation = Operation {
//         size: background.size,
//         images: vec![background, foreground],
//         should_premultiply: true,
//     };

//     let processor = Processor::new().ok().unwrap();
//     let result = processor.composite(&operation).ok().unwrap();

//     io::save_image(result, "/tmp/resize.png");
// }

// #[test]
// fn compositing_three_images_including_negative_offset() {
//     let mut background = Image::color(
//         Size {
//             width: 3,
//             height: 4,
//         },
//         Color::from_u32(0xffffff),
//     );
//     background.position = Point { x: 0.0, y: -1.0 };
//     background.opacity = 0.9;

//     let mut blue_image = Image::color(
//         Size {
//             width: 3,
//             height: 1,
//         },
//         Color::from_u32(0x0000ff),
//     );
//     blue_image.position = Point { x: 0.0, y: 1.0 };

//     let mut magenta_image = Image::color(
//         Size {
//             width: 1,
//             height: 1,
//         },
//         Color::from_u32(0xff00ff),
//     );
//     magenta_image.position = Point { x: 1.0, y: 3.0 };

//     let canvas_size = Size {
//         width: 3,
//         height: 4,
//     };

//     let operation = Operation {
//         size: canvas_size,
//         images: vec![background, blue_image, magenta_image],
//         should_premultiply: true,
//     };

//     let processor = Processor::new().ok().unwrap();
//     let result = processor.composite(&operation).ok().unwrap();

//     let output_buffer =
//         image::ImageBuffer::from_raw(canvas_size.width, canvas_size.height, result.data).unwrap();

//     output_buffer
//         .save(format!("/tmp/{}.png", "comp-with-negative-offset"))
//         .ok();

//     let expected_image = image::open("tests/images/comp-with-negative-offset.png")
//         .unwrap()
//         .to_rgba8();

//     let images_are_equal = images_are_equal(output_buffer, expected_image);
//     assert!(images_are_equal);
// }

#[test]
fn compositing_two_semitransparent_images() {
    let background = Image::color(
        &Color::from_rgb_u32(0xffffff),
        Size {
            width: 3,
            height: 4,
        },
    );
    let mut background = Layer::new(background, Point::zero());
    background.opacity = 0.2;

    let blue_image = Image::color(
        &Color::from_rgb_u32(0x0000ff),
        Size {
            width: 1,
            height: 1,
        },
    );
    let mut blue_layer = Layer::new(blue_image, Point { x: 1.0, y: 1.0 });
    blue_layer.opacity = 0.5;

    let canvas_size = Size {
        width: 3,
        height: 4,
    };

    let operation = Operation::new(vec![background, blue_layer], canvas_size);

    let result = composite(&operation);

    result
        .save(format!("/tmp/{}.png", "comp-two-semitransparent").as_str())
        .unwrap();

    let expected_image = Image::open("tests/images/comp-two-semitransparent.png").unwrap();

    let images_are_equal = result.appears_equal_to(&expected_image);
    assert!(images_are_equal);
}

/// Runs a blend mode test.
fn run_blend_mode_test(blend_mode: BlendMode, opacity: f32) {
    let position = Point::zero();
    let output_name = format!("mountain-{}-{}", blend_mode.as_str(), opacity * 100.0);
    run_blend_mode_test_with_position(
        blend_mode,
        position,
        position,
        1.0,
        opacity,
        output_name,
        None,
    );
}

/// Runs a blend mode test with an offset.
fn run_offset_blend_mode_test(blend_mode: BlendMode, opacity: f32) {
    let base_position = Point { x: 20.0, y: 10.0 };
    let blend_position = Point { x: -20.0, y: 40.0 };
    let output_name = format!(
        "mountain-offset-{}-{}",
        blend_mode.as_str(),
        opacity * 100.0
    );
    run_blend_mode_test_with_position(
        blend_mode,
        base_position,
        blend_position,
        1.0,
        opacity,
        output_name.to_string(),
        None,
    );
}

/// Runs a blend mode test.
fn run_blend_mode_test_with_position(
    blend_mode: BlendMode,
    base_position: Point<f32>,
    blend_position: Point<f32>,
    base_opacity: f32,
    blend_opacity: f32,
    output_name: String,
    container_size: Option<Size<u32>>,
) {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/images/mountain.png");
    let input_image = Image::open(path).unwrap();
    let width = input_image.size.width;
    let height = input_image.size.height;

    let color_image = Image::color(&Color::from_rgb_u32(0x00f5ac), input_image.size);

    let size = Size { width, height };
    let mut layer = Layer::new(input_image, base_position);
    layer.opacity = base_opacity;

    let mut color_layer = Layer::new(color_image, blend_position);
    color_layer.blend_mode = blend_mode;
    color_layer.opacity = blend_opacity;

    let size = container_size.unwrap_or(size);
    let layers = vec![layer, color_layer];

    let operation = Operation::new(layers, size);

    let result = composite(&operation);

    result
        .save(format!("/tmp/{}.png", output_name).as_str())
        .unwrap();

    let directory = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let file_path = format!("{}/tests/images/{}.png", directory, output_name);
    println!("{}", file_path);

    let expected_image = Image::open(file_path).unwrap();

    assert!(result.appears_equal_to(&expected_image));
}

#[test]
fn test_compositing_with_offset() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/images/avatar.png");
    let avatar = Image::open(path).unwrap();

    let mut color_image = Image::color(&Color::from_rgb_u32(0xef5400), avatar.size);

    let layer = Layer::new(avatar, Point { x: -5.0, y: -3.0 });

    composite::draw_layer_over_image(&mut color_image, &layer);

    // color_image.save("/tmp/avatar-with-offset.png").unwrap();

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/images/avatar-with-offset.png");
    let expected_image = Image::open(path).unwrap();

    assert!(color_image.appears_equal_to(&expected_image));
}
