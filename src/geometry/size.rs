use core::ops::Add;
use num_traits::{Float, Num, One, Zero};
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, serde::Deserialize, serde::Serialize)]
/// Represents a size.
pub struct Size<T: Num> {
    /// The width.
    pub width: T,
    /// The height.
    pub height: T,
}

impl<T: Num + Zero> Size<T> {
    /// Creates a new point with 0 for the width and height.
    pub fn zero() -> Size<T> {
        return Self {
            width: T::zero(),
            height: T::zero(),
        };
    }
}

impl<T: Float> Size<T> {
    /// Returns the size rounded and as an integer type.
    pub fn rounded(&self) -> Size<i32> {
        Size {
            width: self.width.round().to_i32().unwrap(),
            height: self.height.round().to_i32().unwrap(),
        }
    }
}

impl<T> One for Size<T>
where
    T: Num + One,
{
    fn one() -> Self {
        Self {
            width: T::one(),
            height: T::one(),
        }
    }
}

impl<T> Mul for Size<T>
where
    T: Num + Mul,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            width: self.width * rhs.width,
            height: self.height * rhs.height,
        }
    }
}

impl<T> Add for Size<T>
where
    T: Num + Add,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Size {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl<T> Zero for Size<T>
where
    T: Num + Zero,
{
    fn zero() -> Self {
        Size {
            width: T::zero(),
            height: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.width.is_zero() && self.height.is_zero()
    }
}

impl From<Size<u16>> for Size<u32> {
    fn from(value: Size<u16>) -> Self {
        Size {
            width: value.width as u32,
            height: value.height as u32,
        }
    }
}

impl From<Size<u32>> for Size<f32> {
    fn from(value: Size<u32>) -> Self {
        Size {
            width: value.width as f32,
            height: value.height as f32,
        }
    }
}

impl From<Size<u32>> for Size<u16> {
    fn from(value: Size<u32>) -> Self {
        Size {
            width: value.width as u16,
            height: value.height as u16,
        }
    }
}

impl From<Size<u16>> for Size<f32> {
    fn from(value: Size<u16>) -> Self {
        Self {
            width: value.width as f32,
            height: value.height as f32,
        }
    }
}

impl From<Size<i32>> for Size<f32> {
    fn from(value: Size<i32>) -> Self {
        Self {
            width: value.width as f32,
            height: value.height as f32,
        }
    }
}

impl From<Size<f32>> for Size<i32> {
    fn from(value: Size<f32>) -> Self {
        Self {
            width: value.width as i32,
            height: value.height as i32,
        }
    }
}

impl From<Size<f32>> for Size<u32> {
    fn from(value: Size<f32>) -> Self {
        Self {
            width: value.width as u32,
            height: value.height as u32,
        }
    }
}

impl From<Size<f32>> for Size<f64> {
    fn from(value: Size<f32>) -> Self {
        Self {
            width: value.width as f64,
            height: value.height as f64,
        }
    }
}

impl From<Size<f64>> for Size<f32> {
    fn from(value: Size<f64>) -> Self {
        Self {
            width: value.width as f32,
            height: value.height as f32,
        }
    }
}

impl From<Size<i32>> for Size<u32> {
    fn from(value: Size<i32>) -> Self {
        let mut width = value.width;
        if width < 0 {
            width *= -1;
        }
        let mut height = value.height;
        if height < 0 {
            height *= -1;
        }
        Self {
            width: width as u32,
            height: height as u32,
        }
    }
}

impl From<Size<u32>> for Size<i32> {
    fn from(value: Size<u32>) -> Self {
        Self {
            width: value.width as i32,
            height: value.height as i32,
        }
    }
}

impl<T: Num> From<[T; 2]> for Size<T>
where
    T: Copy,
{
    fn from(dimensions: [T; 2]) -> Self {
        Size {
            width: dimensions[0],
            height: dimensions[1],
        }
    }
}

impl<T: Num> Size<T>
where
    T: Copy,
{
    /// Returns the point as an array.
    pub fn to_array(&self) -> [T; 2] {
        [self.width, self.height]
    }
}

// SERIALISATION

impl<T> Size<T>
where
    T: Num + Copy + serde::Serialize,
{
    /// Returns the rectangle as a JSON array.
    pub fn to_json_array(&self) -> Result<String, serde_json::Error> {
        let array = self.to_array();
        serde_json::to_string(&array)
    }
}
