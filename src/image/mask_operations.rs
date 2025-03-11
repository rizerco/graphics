use crate::{
    composite::{self, Layer, Operation},
    BlendMode, Image, Mask, Point,
};

/// Deletes the pixels in the image within the supplied mask image.
pub fn delete_pixels(image: &mut Image, mask: &Mask) {
    match mask {
        Mask::Bounded {
            image: mask_image,
            origin,
        } => {
            let base_layer = Layer::new(image, Point::zero());
            let mut blend_layer = Layer::new(mask_image, (*origin).into());
            blend_layer.blend_mode = BlendMode::DestinationOut;

            let operation = Operation::new(vec![base_layer, blend_layer], image.size);
            *image = composite::composite(&operation);
        }
        Mask::Tiled { image, offset } => todo!(),
    }
}

/// Returns the image that intersects the supplied mask.
pub fn subimage(image: &Image, mask: &Mask) -> Image {
    match mask {
        Mask::Bounded {
            image: mask_image,
            origin,
        } => {
            let base_origin = *origin * -1;
            let base_layer = Layer::new(image, base_origin.into());
            let mut blend_layer = Layer::new(mask_image, Point::zero());
            blend_layer.blend_mode = BlendMode::DestinationIn;

            let operation = Operation::new(vec![base_layer, blend_layer], mask_image.size.into());
            composite::composite(&operation)
        }
        Mask::Tiled { image, offset } => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{Image, Mask, Rect};

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
        let mask = Mask::bounded(mask_image, bounding_box);

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
        let mask = Mask::bounded(mask_image, bounding_box);

        let result = super::subimage(&image, &mask);

        // result.save("/tmp/subimage.png").unwrap();

        assert!(result.appears_equal_to(&expected_image));
    }
}
