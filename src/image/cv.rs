use crate::Image;

impl Image {
    /// Returns the image data in a format suitable for a `CVPixelBuffer` on
    /// Apple platforms.
    pub fn pixel_buffer_data(&self) -> Vec<u8> {
        let alignment = 64;
        let remainder = self.bytes_per_row % alignment;
        let output_bytes_per_row = if remainder == 0 {
            self.bytes_per_row
        } else {
            self.bytes_per_row + alignment - remainder
        } as usize;
        let height = self.size.height as usize;
        let output_size = output_bytes_per_row * height;
        let mut output = vec![0; output_size];

        // Rayon can’t beat the standard implementation for a 4k image.

        // output.par_iter_mut().enumerate().for_each(|(index, byte)| {
        //     let y = index / output_bytes_per_row;
        //     let byte_postion = index - output_bytes_per_row * y;
        //     let x = byte_postion / 4;
        //     if x < width {
        //         // The even bytes, green and alpha, don’t need
        //         // to change position.
        //         let source_offset = y * self.bytes_per_row as usize + byte_postion;
        //         if (byte_postion + 1) % 2 == 0 {
        //             *byte = self.data[source_offset];
        //         } else if (byte_postion + 1) % 3 == 0 {
        //             // Red
        //             *byte = self.data[source_offset - 2];
        //         } else {
        //             // Blue
        //             *byte = self.data[source_offset + 2];
        //         }
        //     };
        // });

        for y in 0..height {
            for x in 0..self.size.width as usize {
                let source_offset = y * self.bytes_per_row as usize + x * 4;
                let output_offset = y * output_bytes_per_row + x * 4;
                output[output_offset + 0] = self.data[source_offset + 2]; // Blue
                output[output_offset + 1] = self.data[source_offset + 1]; // Green
                output[output_offset + 2] = self.data[source_offset + 0]; // Red
                output[output_offset + 3] = self.data[source_offset + 3]; // Alpha
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use crate::{Color, Size};

    use super::*;

    #[test]
    fn pixel_buffer_data() {
        let image = Image::color(
            &Color {
                red: 0xad,
                green: 0xde,
                blue: 0x19,
                alpha: 0xff,
            },
            Size {
                width: 13,
                height: 2,
            },
        );

        let result = image.pixel_buffer_data();

        assert_eq!(result.len(), 128);

        assert_eq!(result[0], 0x19);
        assert_eq!(result[1], 0xde);
        assert_eq!(result[2], 0xad);
        assert_eq!(result[3], 0xff);
    }

    // #[test]
    // fn pixel_buffer_data_performance() {
    //     let image = Image::color(
    //         &Color {
    //             red: 0xad,
    //             green: 0xde,
    //             blue: 0x19,
    //             alpha: 0xff,
    //         },
    //         Size {
    //             width: 4000,
    //             height: 2000,
    //         },
    //     );

    //     let now = std::time::Instant::now();
    //     let result = image.pixel_buffer_data();

    //     println!("time taken: {:?}", now.elapsed());

    //     panic!();
    // }
}
