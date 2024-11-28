use std::collections::HashSet;

use crate::{Color, Point};

use super::Image;

impl Image {
    /// Extracts the colours from an image.
    pub fn colors(&self) -> HashSet<Color> {
        let mut colors = HashSet::new();

        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let location = Point { x, y };
                if let Some(color) = self.pixel_color(location.into()) {
                    colors.insert(color);
                }
            }
        }
        colors
    }
}

#[cfg(test)]
mod tests {
    use crate::{Color, Image};

    #[test]
    fn colors_in_avatar() {
        let image = Image::open("tests/images/avatar.png").unwrap();
        let colors = image.colors();

        assert_eq!(colors.len(), 7);

        assert!(colors.contains(&Color::from_rgb_u32(0xc0cbdc)));
        assert!(colors.contains(&Color::from_rgb_u32(0xffffff)));
        assert!(colors.contains(&Color::from_rgb_u32(0xe8b796)));
        assert!(colors.contains(&Color::from_rgb_u32(0xc28569)));
        assert!(colors.contains(&Color::from_rgb_u32(0xb86f50)));
        assert!(colors.contains(&Color::from_rgb_u32(0x262b44)));
        assert!(colors.contains(&Color::from_rgb_u32(0x733e39)));
    }

    #[test]
    fn colors_in_gerbil() {
        let image = Image::open("tests/images/gerbil.jpg").unwrap();
        let colors = image.colors();

        assert_eq!(colors.len(), 37048);
    }
}
