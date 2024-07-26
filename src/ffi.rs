#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_ulong;

pub type dssim_px_t = f32;

#[link(name = "Accelerate", kind = "framework")]
#[cfg(target_vendor = "apple")]
extern "C" {
    /// Reorder color channels within the buffer according to the permute map.
    ///
    /// For each pixel in src, do the following:
    /// ```c
    /// Pixel_8888 srcPixel, result;
    /// for( int i = 0; i < 4; i++ )
    /// result[i] = srcPixel[ permuteMap[i] ];
    /// ```
    ///
    /// The src buffer must be at least as large as the dest buffer in each dimension. (src.height >= dest.height && src.width >= dest.width)
    ///
    /// This function can work in place provided the following are true:
    /// For each buffer "buf" that overlaps with dest, buf->data must be equal to dest->data and buf->rowBytes >= dest->rowBytes
    /// If an overlapping buffer has a different rowBytes from dest, kvImageDoNotTile must be also passed in the flags
    ///
    /// This function may be used with any 4 channel 8-bit/channel format, such as RGBA8888, BGRA8888 or AYUV8888.
    ///
    /// `src`
    /// A pointer to a valid and initialized vImage_Buffer struct, that points to a buffer containing the source pixels.
    ///
    /// `dest`
    /// A pointer to a valid and initialized vImage_Buffer struct, that points to a buffer containing destination pixels.
    ///
    /// `permuteMap`
    /// The map describing the permutation of the 4 color channels.
    /// Each value in the map must be 0,1,2, or 3.  A map of 0,1,2,3
    /// is a copy from src->dest while a map of 3,2,1,0 is permutes
    /// ARGB -> BGRA.  Providing a map value greater than 3 will
    /// result in the return of error kvImageInvalidParameter.
    ///
    /// `flags`
    /// kvImageNoFlags                     Default operation
    /// kvImageDoNotTile                   Disable internal multithreading.
    ///
    /// # Return values
    /// `kvImageNoError`                   Success
    /// `kvImageInvalidParameter`          When permuteMap > 3, which is invalid.
    /// `kvImageRoiLargerThanInputBuffer`  The height and width of the destination must be less than or equal to the height and width of the src buffer, respectively.
    /// */
    pub(crate) fn vImagePermuteChannels_ARGB8888(
        src: *const vImage_Buffer<*const u8>,
        dest: *mut vImage_Buffer<*mut u8>,
        permuteMap: *const u8,
        flags: vImage_Flags,
    ) -> vImage_Error;
}

pub type vImagePixelCount = c_ulong;
pub type vImage_Error = isize;

#[repr(u32)]
/// The values here indicate bits in a vImage_Flags bit field.
/// Other bits are reserved for future use.
/// Some flags are mutually exclusive. You can not have more
/// than one bit from this set set at the same time:
/// { kvImageCopyInPlace, kvImageBackgroundColorFill, kvImageEdgeExtend, kvImageTruncateKernel }
/// all unused flags bits must be set to 0
/// Not all flags are allowed by all functions.
pub enum vImage_Flags {
    kvImageNoFlags = 0,

    /// Operate on red, green and blue channels only. Alpha is copied from source
    /// to destination. For Interleaved formats only.
    kvImageLeaveAlphaUnchanged = 1,

    /// Copy edge pixels. Convolution Only.
    kvImageCopyInPlace = 2,

    /// Use the background color for missing pixels.
    kvImageBackgroundColorFill = 4,

    /// Use the nearest pixel for missing pixels.
    kvImageEdgeExtend = 8,

    /// Pass to turn off internal tiling and disable internal multithreading. Use this if
    /// you want to do your own tiling, or to use the Min/Max filters in place.
    kvImageDoNotTile = 16,

    /// Use a higher quality, slower resampling filter for Geometry operations
    /// (shear, scale, rotate, affine transform, etc.)
    kvImageHighQualityResampling = 32,

    /// Use only the part of the kernel that overlaps the image. For integer kernels,
    /// real_divisor = divisor * (sum of used kernel elements) / (sum of kernel elements).
    /// This should preserve image brightness at the edges. Convolution only.
    kvImageTruncateKernel = 64,

    /// The function will return the number of bytes required for the temp buffer.
    /// If this value is negative, it is an error, per standard usage.
    kvImageGetTempBufferSize = 128,

    /// Some functions such as vImageConverter_CreateWithCGImageFormat have so many possible error conditions
    /// that developers may need more help than a simple error code to diagnose problems. When this
    /// flag is set and an error is encountered, an informative error message will be logged to the Apple
    /// System Logger (ASL).  The output should be visible in Console.app.
    kvImagePrintDiagnosticsToConsole = 256,

    /// Pass this flag to prevent vImage from allocating additional storage.
    kvImageNoAllocate = 512,

    /// Use methods that are HDR-aware, capable of providing correct results for input images with pixel values
    /// outside the otherwise limited (typically [-2,2]) range. This may be slower.
    kvImageHDRContent = 1024,
}

#[repr(C)]
/// The vImage_Buffer describes a rectangular region within a regular array of pixels. It may describe
/// the entire image, or just a sub rectangle of it.  The vImage_Buffer struct is not a complete description
/// of an image. Other aspects like pixel format, color space, channel ordering, etc. are generally given
/// by the names of functions that operate on the vImage_Buffer or by parameters passed to those functions.
/// A vImage_Buffer may contain multiple color channels interleaved with one another, or a single color channel
/// (or alpha) as a planar buffer.  vImage_Buffers are often initialized directly by you, by setting fields
/// to appropriate values to point to image data you already own. Convenience methods are also available as
/// vImageBuffer_Init, vImageBuffer_InitWithCGImage and vImageBuffer_InitWithCVPixelBuffer
pub struct vImage_Buffer<T> {
    /// A pointer to the top left corner of the buffer contain image pixels.
    pub data: T,
    /// The number of pixels in a column of the image.
    pub height: vImagePixelCount,
    /// The number of visible pixels in a row of an image (excluding padding at the ends of rows)
    pub width: vImagePixelCount,
    /// The number of bytes from a pixel to the next pixel down in the same column.
    pub rowBytes: usize,
}
