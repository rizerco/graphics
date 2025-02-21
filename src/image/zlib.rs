use std::io::{Cursor, Read, Write};

use file_stream::{read::FileStreamReader, write::FileStreamWriter};
use flate2::{bufread::ZlibDecoder, write::ZlibEncoder, Compression};

use crate::Size;

use super::Image;

/// The header for the custom image file format.
pub static HEADER: [u8; 7] = [0x5A, 0x4C, 0x49, 0x42, 0x49, 0x4D, 0x47];

/// Returns whether or not the supplied data is a
/// Zlib image.
pub fn is_zlib_image(data: &[u8]) -> bool {
    if data.len() < 7 {
        return false;
    }
    data[0..7] == HEADER
}

impl Image {
    /// Creates an image from Zlib encoded data.
    pub fn from_zlib_image_data(data: Vec<u8>) -> anyhow::Result<Image> {
        let mut file_stream = FileStreamReader::from_data(data)?;
        // Skipping the header, version, and format.
        file_stream.skip_bytes(9)?;
        let width = file_stream.read_be()?;
        let height = file_stream.read_be()?;
        let size = Size { width, height };
        let bytes_per_row = file_stream.read_be()?;

        let encoded_data = file_stream.read_remaining_data()?;
        let cursor = Cursor::new(encoded_data);
        let mut decoder = ZlibDecoder::new(cursor);
        let mut data = Vec::new();
        // Ignoring the result because sometimes the
        // data does not have the checksum, which will
        // produce an error.
        let _ = decoder.read_to_end(&mut data);
        let image = Image {
            data,
            size,
            bytes_per_row,
        };
        Ok(image)
    }

    /// Returns the image data in a custom Zlib backed format.
    pub fn zlib_image_data(&self) -> anyhow::Result<Vec<u8>> {
        let mut file_stream = FileStreamWriter::new();
        file_stream.write_bytes(&HEADER)?;
        // Version number.
        file_stream.write_be(&0u8)?;
        // Write a placeholder for the image format.
        file_stream.write_be(&0u8)?;
        file_stream.write_be(&self.size.width)?;
        file_stream.write_be(&self.size.height)?;
        file_stream.write_be(&self.bytes_per_row)?;

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
        encoder.write_all(&self.data)?;
        let encoded_data = encoder.finish()?;
        file_stream.write_bytes(&encoded_data)?;

        Ok(file_stream.data().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::Image;

    #[test]
    fn zlib_image() {
        let image = Image::open("tests/images/mountain.png").unwrap();

        let encoded = image.zlib_image_data().unwrap();

        assert!(super::is_zlib_image(&encoded));

        let decoded = Image::from_zlib_image_data(encoded).unwrap();

        assert!(image.appears_equal_to(&decoded));
    }
}
