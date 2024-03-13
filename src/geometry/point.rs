use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use num_traits::{Float, Num, Zero};

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
/// Represents a 2D point.
pub struct Point<T: Num> {
    /// The x-ccordinate.
    pub x: T,
    /// The y-coordinate.
    pub y: T,
}

impl<T: Num + Zero> Point<T> {
    /// Creates a new point with 0 for the x and y.
    pub fn zero() -> Point<T> {
        return Point {
            x: T::zero(),
            y: T::zero(),
        };
    }
}

impl From<Point<i16>> for Point<f32> {
    fn from(value: Point<i16>) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl From<Point<i32>> for Point<f32> {
    fn from(value: Point<i32>) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl From<Point<i32>> for Point<u32> {
    fn from(value: Point<i32>) -> Self {
        Self {
            x: value.x as u32,
            y: value.y as u32,
        }
    }
}

impl From<Point<i32>> for Point<usize> {
    fn from(value: Point<i32>) -> Self {
        Self {
            x: value.x as usize,
            y: value.y as usize,
        }
    }
}

impl From<Point<u32>> for Point<i32> {
    fn from(value: Point<u32>) -> Self {
        Self {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

impl From<Point<u32>> for Point<f32> {
    fn from(value: Point<u32>) -> Self {
        Self {
            x: value.x as f32,
            y: value.y as f32,
        }
    }
}

impl From<Point<f32>> for Point<u32> {
    fn from(value: Point<f32>) -> Self {
        Self {
            x: value.x as u32,
            y: value.y as u32,
        }
    }
}

impl From<Point<f32>> for Point<i32> {
    fn from(value: Point<f32>) -> Self {
        Self {
            x: value.x as i32,
            y: value.y as i32,
        }
    }
}

impl<T, U> From<[T; 2]> for Point<U>
where
    T: Num + Into<U> + Copy,
    U: Num + Copy,
{
    fn from(coords: [T; 2]) -> Self {
        Point {
            x: coords[0].into(),
            y: coords[1].into(),
        }
    }
}

impl<T: Num> Point<T>
where
    T: Copy,
{
    /// Returns the point as an array.
    pub fn to_array(&self) -> [T; 2] {
        [self.x, self.y]
    }
}

impl<T: Float> Point<T> {
    /// Returns the point rounded and as an integer type.
    pub fn rounded(&self) -> Point<i32> {
        Point {
            x: self.x.round().to_i32().unwrap(),
            y: self.y.round().to_i32().unwrap(),
        }
    }

    /// Returns the point floored and as an integer type.
    pub fn floored(&self) -> Point<i32> {
        Point {
            x: self.x.floor().to_i32().unwrap(),
            y: self.y.floor().to_i32().unwrap(),
        }
    }
}

impl<T> Point<T>
where
    T: Num + MulAssign<i32>,
{
    /// Inverts this point, multiplying x and y by -1;
    pub fn invert(&mut self) {
        self.x *= -1;
        self.y *= -1;
    }
}

// MATHS

impl<T> Add for Point<T>
where
    T: Num,
{
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign for Point<T>
where
    T: Num + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> Sub for Point<T>
where
    T: Num,
{
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> SubAssign for Point<T>
where
    T: Num + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Mul<T> for Point<T>
where
    T: Copy + Num + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// MARK: Actions

impl<T> Point<T>
where
    T: Float,
{
    /// Returns the point rotated by an angle about a point.
    pub fn rotated(self, angle: T, point: Point<T>) -> Point<T> {
        let translated_point = self - point;
        let rotated_x =
            translated_point.x * Float::cos(angle) - translated_point.y * Float::sin(angle);
        let rotated_y =
            translated_point.x * Float::sin(angle) + translated_point.y * Float::cos(angle);
        // Rounding the values as they can be a little off.
        let rounding_value = T::from(10000.0).unwrap();
        let rotated_x = T::round(rotated_x * rounding_value) / rounding_value;
        let rotated_y = T::round(rotated_y * rounding_value) / rounding_value;
        let rotated_point = Point {
            x: rotated_x,
            y: rotated_y,
        };
        rotated_point + point
    }
}

// SERIALISATION

impl<T> Point<T>
where
    T: Num + Copy + serde::Serialize,
{
    /// Returns the rectangle as a JSON array.
    pub fn to_json_array(&self) -> Result<String, serde_json::Error> {
        let array = self.to_array();
        serde_json::to_string(&array)
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_rotated_90_degress() {
        let point = Point { x: 13.0, y: 3.0 };
        // I don’t know why this needs to be negative.
        let angle = -std::f32::consts::PI * 0.5;
        dbg!(angle);
        let result = point.rotated(angle, Point::zero());
        let expected_result = Point { x: 3.0, y: -13.0 };
        assert_eq!(result, expected_result);
    }
}
