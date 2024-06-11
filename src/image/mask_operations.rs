use crate::{
    composite::{self, Layer, Operation},
    BlendMode, Image, Mask, Point,
};

/// Deletes the pixels in the image within the supplied mask image.
pub fn delete_pixels(image: &mut Image, mask: &dyn Mask) {
    let base_layer = Layer::new(image, Point::zero());
    let mut blend_layer = Layer::new(mask.image(), mask.bounding_box().origin.into());
    blend_layer.blend_mode = BlendMode::DestinationOut;

    let operation = Operation::new(vec![base_layer, blend_layer], image.size);
    *image = composite::composite(&operation);
}

/// Returns the image that intersects the supplied mask.
pub fn subimage(image: &Image, mask: &dyn Mask) -> Image {
    let base_origin = mask.bounding_box().origin * -1;
    let base_layer = Layer::new(image, base_origin.into());
    let mut blend_layer = Layer::new(mask.image(), Point::zero());
    blend_layer.blend_mode = BlendMode::DestinationIn;

    let operation = Operation::new(
        vec![base_layer, blend_layer],
        mask.bounding_box().size.into(),
    );
    composite::composite(&operation)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{Image, Mask, Rect};

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
    fn delete_pixels() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/sf2-1x.png");
        let mut image = Image::open(path).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/sf2-1x-subimage-mask.png");
        let mask_image = Image::open(path).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/sf2-1x-erased.png");
        let expected_image = Image::open(path).unwrap();

        let bounding_box = Rect::new(5, 100, 15, 15);
        let mask = TestMask {
            image: mask_image,
            bounding_box,
        };

        super::delete_pixels(&mut image, &mask);

        // image.save("/tmp/delete_pixels.png").unwrap();

        assert!(image.appears_equal_to(&expected_image));
    }

    #[test]
    fn subimage() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/sf2-1x.png");
        let image = Image::open(path).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/sf2-1x-subimage-mask.png");
        let mask_image = Image::open(path).unwrap();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/images/sf2-1x-subimage.png");
        let expected_image = Image::open(path).unwrap();

        let bounding_box = Rect::new(5, 100, 15, 15);
        let mask = TestMask {
            image: mask_image,
            bounding_box,
        };

        let result = super::subimage(&image, &mask);

        // result.save("/tmp/subimage.png").unwrap();

        assert!(result.appears_equal_to(&expected_image));
    }
}
