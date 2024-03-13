use crate::{Image, Point, Rect, Size};

impl Image {
    /// Flips an image horizontally.
    pub fn flip_horizontally(&mut self) {
        let width = self.size.width;
        for row in 0..self.size.height {
            for column in 0..(width / 2) {
                let left_location = Point { x: column, y: row }.into();
                let Some(left_pixel) = self.pixel_color(left_location) else {
                    continue;
                };
                let right_location = Point {
                    x: width - 1 - column,
                    y: row,
                }
                .into();
                let Some(right_pixel) = self.pixel_color(right_location) else {
                    continue;
                };
                self.set_pixel_color(left_pixel, right_location.into());
                self.set_pixel_color(right_pixel, left_location.into());
            }
        }
    }

    /// Flips an image vertically.
    pub fn flip_vertically(&mut self) {
        let height = self.size.height;
        for column in 0..self.size.width {
            for row in 0..(height / 2) {
                let top_location = Point { x: column, y: row }.into();
                let Some(top_pixel) = self.pixel_color(top_location) else {
                    continue;
                };
                let bottom_location = Point {
                    x: column,
                    y: height - 1 - row,
                }
                .into();
                let Some(bottom_pixel) = self.pixel_color(bottom_location) else {
                    continue;
                };
                self.set_pixel_color(top_pixel, bottom_location.into());
                self.set_pixel_color(bottom_pixel, top_location.into());
            }
        }
    }

    /// Resizes an image using the nearest neighbour algorithm.
    pub fn resize_nearest_neighbor(&mut self, new_size: Size<u32>) {
        let mut new_image = Image::empty(new_size);

        let x_scale = self.size.width as f32 / new_size.width as f32;
        let y_scale = self.size.height as f32 / new_size.height as f32;

        for y in 0..new_size.height {
            for x in 0..new_size.width {
                // Using `floor` to match Aseprite’s behaviour.
                // I’m not sure what, if anything, is correct.
                let sample_x = (x as f32 * x_scale).floor() as i32;
                let sample_y = (y as f32 * y_scale).floor() as i32;
                let location = Point {
                    x: sample_x,
                    y: sample_y,
                };
                let Some(color) = self.pixel_color(location) else {
                    continue;
                };
                let location = Point { x, y }.into();
                new_image.set_pixel_color(color, location);
            }
        }

        *self = new_image;
    }

    /// Rotates the image using the nearest neighbour algorithm.
    /// Returns the offset for the new origin.
    pub fn rotate_nearest_neighbor(&mut self, angle: f32, center: Point<f32>) -> Point<i32> {
        let bounds = Rect {
            origin: Point::zero(),
            size: self.size.into(),
        };
        let new_bounds = bounds.rotated(angle, center);
        let new_size = Size {
            width: new_bounds.size.width.ceil() as u32,
            height: new_bounds.size.height.ceil() as u32,
        };

        let mut new_image = Image::empty(new_size);

        let offset = Point {
            x: -new_bounds.origin.x,
            y: -new_bounds.origin.y,
        };

        for y in 0..new_image.size.height {
            for x in 0..new_image.size.width {
                let location = Point { x, y };
                let rotated_location: Point<f32> = location.into();
                let rotated_location = rotated_location + Point { x: 0.5, y: 0.5 };
                let rotated_location = rotated_location.rotated(-angle, center);
                let rotated_location = rotated_location.floored();
                let Some(color) = self.pixel_color(rotated_location) else {
                    continue;
                };
                new_image.set_pixel_color(color, location + offset.into());
            }
        }

        *self = new_image;

        offset.into()
    }
}
