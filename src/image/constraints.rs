use crate::Size;

/// Constraints on an image’s size.
pub struct ImageConstraints {
    /// The maximum size of an image.
    pub maximum_size: Option<Size<u32>>,
    /// The maximum resolution, specified as number of pixels.
    pub maximum_resolution: Option<u32>,
}

impl ImageConstraints {
    /// Returns whether or not a give size exceeds these constraints.
    pub fn is_exceeded_by(&self, size: &Size<u32>) -> bool {
        if let Some(max_size) = self.maximum_size {
            if size.width as u32 > max_size.width || size.height as u32 > max_size.height {
                return true;
            }
        }

        if let Some(max_resolution) = self.maximum_resolution {
            let resolution = size.width as u32 * size.height as u32;
            if resolution > max_resolution {
                return true;
            }
        }

        false
    }
}
