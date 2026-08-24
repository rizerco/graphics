use image::{RgbaImage, imageops};

/// Fixes the orientation of an image.
pub(super) fn fix_orientation(image: &mut RgbaImage, orientation: u32) {
    if orientation > 8 {
        return;
    }

    if orientation >= 5 {
        *image = imageops::rotate90(image);
        imageops::flip_horizontal_in_place(image);
    }

    if orientation == 3 || orientation == 4 || orientation == 7 || orientation == 8 {
        imageops::rotate180_in_place(image);
    }

    if orientation % 2 == 0 {
        imageops::flip_horizontal_in_place(image);
    }
}
