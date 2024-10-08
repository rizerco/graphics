pub mod cv;
mod mask_operations;
pub mod transformation;

pub use mask_operations::*;
use tiff::encoder::compression::Compression;
use tiff::encoder::{colortype, TiffEncoder};

use std::cmp::min;
use std::io::Cursor;
use std::path::Path;

use image::{DynamicImage, ImageFormat, RgbaImage};

use crate::composite::{self, Layer};
use crate::{BlendMode, Color, Mask, Point, Rect, Size};

/// The representation of an image for graphics manipulation.
#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Image {
    /// The raw image data.
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    /// The image size.
    pub size: Size<u32>,
    /// The number of bytes per row.
    pub bytes_per_row: u32,
}

// CREATION

impl Image {
    /// Creates a new image with pixel data.
    pub fn new(data: Vec<u8>, size: Size<u32>, bytes_per_row: u32) -> Self {
        Self {
            data,
            size,
            bytes_per_row,
        }
    }
    /// Creates an empty image of a given size.
    pub fn empty(size: Size<u32>) -> Self {
        let bytes_per_row = size.width * 4;
        let data_size = (bytes_per_row * size.height) as usize;
        let data = vec![0u8; data_size];
        Image {
            data,
            size,
            bytes_per_row,
        }
    }

    /// Creates an image with a colour.
    pub fn color(color: &Color, size: Size<u32>) -> Image {
        let bytes_per_row = size.width * 4;
        let data_size = (size.width * size.height) as usize;
        let data = vec![color.red, color.green, color.blue, color.alpha].repeat(data_size);
        Image {
            data,
            size,
            bytes_per_row,
        }
    }
}

// IMAGE FILE INTEGRATION

impl Image {
    /// Creates a new image from file data.
    pub fn from_file_data(data: &[u8]) -> anyhow::Result<Self> {
        let dyanic_image = image::load_from_memory(data)?;
        Self::from_dynamic_image(dyanic_image)
    }

    /// Opens an image file.
    pub fn open<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let dynamic_image = image::open(path)?;
        Self::from_dynamic_image(dynamic_image)
    }

    /// Creates a new image from an RgbaImage.
    fn from_rgba_image(input_image: RgbaImage) -> anyhow::Result<Self> {
        let (width, height) = input_image.dimensions();

        if width == 0 || height == 0 {
            anyhow::bail!("Invalid image dimensions.");
        }

        let data: Vec<u8> = input_image.into_vec();
        let bytes_per_row = (data.len() / height as usize) as u32;

        let size = Size { width, height };
        let output = Image {
            data,
            size,
            bytes_per_row,
        };

        Ok(output)
    }

    /// Creates a new image from a DynamicImage.
    fn from_dynamic_image(dynamic_image: DynamicImage) -> anyhow::Result<Self> {
        let input_image = dynamic_image.to_rgba8();
        Self::from_rgba_image(input_image)
    }

    /// Saves the image to a file.
    pub fn save<P>(&self, path: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let size = self.size;
        let data = self.data.clone();
        let output_buffer: image::RgbaImage =
            image::ImageBuffer::from_raw(size.width, size.height, data)
                .ok_or(anyhow::anyhow!("Unable to create image from raw data."))?;
        output_buffer.save(path)?;
        Ok(())
    }

    /// Outputs data for the image in the specified format.
    pub fn file_data(&self, format: ImageFormat) -> anyhow::Result<Vec<u8>> {
        let size = self.size;
        let data = self.data.clone();
        let output_buffer: image::RgbaImage =
            image::ImageBuffer::from_raw(size.width, size.height, data)
                .ok_or(anyhow::anyhow!("Unable to create image from raw data."))?;

        let mut file_data = Vec::new();
        let mut cursor = Cursor::new(&mut file_data);
        output_buffer.write_to(&mut cursor, format)?;
        Ok(file_data)
    }

    /// Outputs data for the image using the TIFF format.
    /// This allows for the compression algorithm to be set.
    pub fn tiff_data<D>(&self, compression: D) -> anyhow::Result<Vec<u8>>
    where
        D: Compression,
    {
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);

        let mut tiff = TiffEncoder::new(&mut cursor)?;
        tiff.write_image_with_compression::<colortype::RGBA8, _>(
            self.size.width,
            self.size.height,
            compression,
            &self.data,
        )?;

        Ok(buffer)
    }

    /// Outputs the data as an image buffer.
    pub fn to_image_buffer(&self) -> anyhow::Result<image::RgbaImage> {
        let size = self.size;
        let data = self.data.clone();
        let output_buffer: image::RgbaImage =
            image::ImageBuffer::from_raw(size.width, size.height, data)
                .ok_or(anyhow::anyhow!("Unable to create image from raw data."))?;
        Ok(output_buffer)
    }
}

// EQUALITY

impl Image {
    /// Returns whether or not the image is transparent.
    pub fn is_transparent(&self) -> bool {
        for y in 0..self.size.height as usize {
            let row_start = y * self.bytes_per_row as usize;
            let row_end = row_start + 4 * self.size.width as usize;

            if self.data[row_start + 3..row_end]
                .iter()
                .step_by(4)
                .any(|&alpha| alpha != 0)
            {
                return false;
            }
        }
        true
    }

    /// Returns whether or not one image appears equal to another.
    /// This is computationally expensive and is only really meant
    /// for use in tests.
    pub fn appears_equal_to(&self, other_image: &Image) -> bool {
        if self.size != other_image.size {
            return false;
        }

        if self.is_transparent() && other_image.is_transparent() {
            return true;
        }

        if self.bytes_per_row == other_image.bytes_per_row {
            if self.data == other_image.data {
                return true;
            }
        }

        let byte_width = (self.size.width * 4) as usize;
        for y in 0..self.size.height {
            let offset = (self.bytes_per_row * y) as usize;
            let other_offset = (other_image.bytes_per_row * y) as usize;

            let visible_range = offset..(offset + byte_width);
            let other_visible_range = other_offset..(other_offset + byte_width);
            let row_data = &self.data[visible_range];
            let other_row_data = &other_image.data[other_visible_range];

            if row_data != other_row_data {
                for x in (0..row_data.len()).step_by(4) {
                    let alpha_index = x + 3;
                    // Check the alpha channels. If both are fully
                    // transparent then they appear equal.
                    if row_data[alpha_index] == 0 && other_row_data[alpha_index] == 0 {
                        continue;
                    }

                    if row_data[x..=alpha_index] != other_row_data[x..=alpha_index] {
                        return false;
                    }
                }
            }
        }

        true
    }
}

// CROPPING

impl Image {
    /// Crops an image to fit a given size, keeping the image at the centre.
    /// The size can be larger or smaller than the image in either dimension.
    pub fn crop_with_offset(&mut self, size: Size<u32>, offset: Point<i32>) -> anyhow::Result<()> {
        let origin = offset;
        if size == self.size && origin == Point::zero() {
            return Ok(());
        }

        let mut image_buffer = self.to_image_buffer()?;

        // TODO: Need to handle negative offsets.
        let result = image::imageops::crop(
            &mut image_buffer,
            offset.x as u32,
            offset.y as u32,
            size.width,
            size.height,
        );

        let result = Image::from_rgba_image(result.to_image())?;
        self.data = result.data;
        self.size = result.size;
        self.bytes_per_row = result.bytes_per_row;

        Ok(())
    }
}

// SAMPLING

impl Image {
    /// Returns the colour of the pixel at a given point.
    pub fn pixel_color(&self, location: Point<i32>) -> Option<Color> {
        let bounding_box = Rect {
            origin: Point::zero(),
            size: self.size.into(),
        };
        if !bounding_box.contains(location) {
            return None;
        }

        let offset = self.bytes_per_row as usize * location.y as usize + location.x as usize * 4;

        if self.data.len() < offset + 4 {
            return None;
        }

        let red = self.data[offset + 0];
        let green = self.data[offset + 1];
        let blue = self.data[offset + 2];
        let alpha = self.data[offset + 3];

        let color = Color {
            red,
            green,
            blue,
            alpha,
        };

        Some(color)
    }

    /// Sets the colour of the pixel at a given point.
    pub fn set_pixel_color(&mut self, color: Color, location: Point<u32>) {
        let bounding_box = Rect::<i32> {
            origin: Point::zero(),
            size: self.size.into(),
        };
        if !bounding_box.contains(location.into()) {
            return;
        }

        let offset = self.bytes_per_row as usize * location.y as usize + location.x as usize * 4;

        if self.data.len() < offset + 4 {
            return;
        }

        self.data[offset + 0] = color.red;
        self.data[offset + 1] = color.green;
        self.data[offset + 2] = color.blue;
        self.data[offset + 3] = color.alpha;
    }
}

// TRIMMING

impl Image {
    /// Trims the transparent pixels from the edge of the image and returns
    /// the new bounding rect relative to the original.
    pub fn trim(&mut self) -> anyhow::Result<Rect<i32>> {
        let container = Rect {
            origin: Point::zero(),
            size: self.size.into(),
        };
        self.trim_in_container(container)
    }

    /// Trims the transparent pixels from the edge of the image and returns
    /// the new bounding rect relative to the original.
    pub fn trim_in_container(&mut self, container: Rect<i32>) -> anyhow::Result<Rect<i32>> {
        let bytes_per_row = self.bytes_per_row as i32;
        let image_size = Size {
            width: self.size.width as i32,
            height: self.size.height as i32,
        };
        let container = container
            .intersection(&Rect {
                origin: Point::zero(),
                size: image_size,
            })
            .ok_or(anyhow::anyhow!("Container is outside of the image bounds."))?;

        let min_x = container.min_x();
        let max_x = container.max_x();
        let min_y = container.min_y();
        let max_y = container.max_y();

        const ALPHA_OFFSET: i32 = 3;

        // Search from the top.
        let mut top = min_y;
        let mut has_found_top = false;

        for y in min_y..max_y {
            let mut row_is_transparent = true;
            for x in min_x..max_x {
                let offset = (bytes_per_row * y) + (x * 4) + ALPHA_OFFSET;

                let alpha = self.data[offset as usize];

                if alpha != 0 {
                    has_found_top = true;
                    row_is_transparent = false;
                    break;
                }
            }

            if row_is_transparent {
                top = y + 1;
            }

            if has_found_top {
                break;
            }
        }

        if top >= max_y {
            anyhow::bail!("The found top is greater than the max Y.");
        }

        // Search from the bottom.
        let mut bottom = max_y;
        let mut has_found_bottom = false;

        for y in (min_y..max_y).rev() {
            let mut row_is_transparent = true;
            for x in min_x..max_x {
                let offset = (bytes_per_row * y) + (x * 4) + ALPHA_OFFSET;

                let alpha = self.data[offset as usize];

                if alpha != 0 {
                    has_found_bottom = true;
                    row_is_transparent = false;
                    break;
                }
            }

            if row_is_transparent {
                bottom = y;
            }

            if has_found_bottom {
                break;
            }
        }

        if bottom <= top {
            anyhow::bail!(
                "The found bottom ({}) is less than the found top ({}).",
                bottom,
                top
            );
        }

        // Search from the left.
        let mut left = min_x;
        let mut has_found_left = false;

        for x in min_x..max_x {
            let mut column_is_transparent = true;
            for y in top..bottom {
                let offset = (bytes_per_row * y) + (x * 4) + ALPHA_OFFSET;

                let alpha = self.data[offset as usize];

                if alpha != 0 {
                    has_found_left = true;
                    column_is_transparent = false;
                    break;
                }
            }

            if column_is_transparent {
                left = x + 1;
            }

            if has_found_left {
                break;
            }
        }

        if left >= max_x {
            anyhow::bail!("The found left edge is greater than the maximum x.");
        }

        // Search from the right.
        let mut right = max_x;
        let mut has_found_right = false;

        for x in (min_x..max_x).rev() {
            let mut column_is_transparent = true;
            for y in top..bottom {
                let offset = (bytes_per_row * y) + (x * 4) + ALPHA_OFFSET;

                let alpha = self.data[offset as usize];

                if alpha != 0 {
                    has_found_right = true;
                    column_is_transparent = false;
                    break;
                }
            }

            if column_is_transparent {
                right = x;
            }

            if has_found_right {
                break;
            }
        }

        if right <= left {
            anyhow::bail!("The found right edge is greater than the found left edge.");
        }

        let size = Size {
            width: right - left,
            height: bottom - top,
        };
        let origin = Point { x: left, y: top };

        let rect = Rect { origin, size };

        self.crop_with_offset(size.into(), origin)?;

        Ok(rect)
    }
}

// PIXEL REPLACEMENT

impl Image {
    /// Draws another image at a location in this image, replacing all the
    /// pixels in the existing image.
    pub fn draw_image_over(&mut self, other_image: &Image, location: Point<i32>) {
        let start_x = if location.x < 0 { 0 } else { location.x as u32 };
        if start_x >= self.size.width {
            return;
        }
        let end_x = other_image.size.width as i32 + location.x;
        if end_x <= 0 {
            return;
        }
        let end_x = end_x as u32;
        let end_x = min(self.size.width, end_x);
        let required_width = (end_x - start_x) as usize;

        let start_y = if location.y < 0 { 0 } else { location.y as u32 };
        if start_y >= self.size.height {
            return;
        }
        let end_y = other_image.size.height as i32 + location.y;
        if end_y <= 0 {
            return;
        }
        let end_y = end_y as u32;
        let end_y = min(self.size.height, end_y);
        let required_height = end_y - start_y;

        let target_y_offset = if location.y < 0 { 0 } else { location.y as u32 };

        // I tried using rayon for this, but with 10,000 rows the performance
        // was a little worse with rayon than without.
        for y in 0..required_height {
            let overlay_y = if location.y < 0 {
                y as i32 - location.y
            } else {
                y as i32
            };
            let overlay_x = if location.x < 0 { -location.x } else { 0 };
            let offset = (overlay_y as u32 * other_image.bytes_per_row) as i32;
            let offset = (offset + overlay_x * 4) as usize;
            let target_offset = ((target_y_offset + y) * self.bytes_per_row) as i32;
            let target_offset = (target_offset + (start_x as i32) * 4) as usize;
            // Using a second loop was a tiny bit faster than splicing the vec.
            for x in 0..required_width * 4 {
                self.data[target_offset + x] = other_image.data[offset + x];
            }
        }
    }

    /// Returns a new image that is the image intersecting
    /// the supplied mask.
    pub fn subimage_masked(&self, mask: &dyn Mask) -> anyhow::Result<Image> {
        let mut result = self.clone();
        result.crop_with_offset(mask.bounding_box().size.into(), mask.bounding_box().origin)?;
        let mut layer = Layer::new(mask.image(), Point::zero());
        layer.blend_mode = BlendMode::DestinationIn;
        composite::draw_layer_over_image(&mut result, &layer);
        Ok(result)
    }

    /// Returns a new image that is a subimage of this image within
    /// the supplied bounds.
    pub fn subimage(&self, region: Rect<i32>) -> anyhow::Result<Image> {
        let mut result = Image::empty(region.size.into());
        for y in 0..region.size.height {
            for x in 0..region.size.width {
                let point = Point {
                    x: region.origin.x + x,
                    y: region.origin.y + y,
                };
                let Some(color) = self.pixel_color(point) else {
                    continue;
                };
                let point = Point { x, y }.into();
                result.set_pixel_color(color, point);
            }
        }
        Ok(result)
    }
}
