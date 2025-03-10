use file_stream::{read::FileStreamReader, write::FileStreamWriter};

use crate::Size;

use super::Image;

impl Image {
    /// Creates an image from raw data.
    pub fn from_raw_data(data: Vec<u8>) -> anyhow::Result<Image> {
        let mut file_stream = FileStreamReader::from_data(data)?;
        // Skipping the format for now.
        file_stream.skip_bytes(1)?;
        let width = file_stream.read_be()?;
        let height = file_stream.read_be()?;
        let size = Size { width, height };
        let bytes_per_row = file_stream.read_be()?;

        let data = file_stream.read_remaining_data()?;
        let image = Image {
            data,
            size,
            bytes_per_row,
        };
        Ok(image)
    }

    /// Returns the image data in a simple raw format.
    pub fn raw_data(&self) -> anyhow::Result<Vec<u8>> {
        let mut file_stream = FileStreamWriter::new();
        // Write a placeholder for the image format.
        file_stream.write_be(&0u8)?;
        file_stream.write_be(&self.size.width)?;
        file_stream.write_be(&self.size.height)?;
        file_stream.write_be(&self.bytes_per_row)?;
        file_stream.write_bytes(&self.data)?;

        Ok(file_stream.data().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::Image;

    #[test]
    fn raw_data() {
        let image = Image::open("tests/images/mountain.png").unwrap();

        let encoded = image.raw_data().unwrap();

        let decoded = Image::from_raw_data(encoded).unwrap();

        assert!(image.appears_equal_to(&decoded));
    }
}
