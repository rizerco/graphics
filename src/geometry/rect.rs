use std::{cmp, ops::AddAssign};

use num_traits::{abs, Float, Num, PrimInt, Signed, Zero};

use crate::{EdgeInsets, Point, Size};

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
/// Represents a rectangle.
pub struct Rect<T: Num> {
    /// The origin.
    pub origin: Point<T>,
    /// The size.
    pub size: Size<T>,
}

impl<T: Num> Rect<T> {
    /// Creates a new rectangle.
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        let origin = Point { x, y };
        let size = Size { width, height };
        Rect { origin, size }
    }
}

impl<T> Rect<T>
where
    T: Num + Ord + PrimInt + Signed + AddAssign,
{
    /// Creates a rectangle containing two points.
    pub fn containing(point_a: &Point<T>, point_b: &Point<T>) -> Self {
        let mut top_left = Point::zero();
        let mut bottom_right = Point::zero();

        if point_a.x <= point_b.x {
            top_left.x = point_a.x;
            bottom_right.x = point_b.x;
        } else {
            top_left.x = point_a.x;
            bottom_right.x = point_b.x;
        }

        if point_a.y <= point_b.y {
            top_left.y = point_a.y;
            bottom_right.y = point_b.y;
        } else {
            top_left.y = point_a.y;
            bottom_right.y = point_b.y;
        }

        let width = bottom_right.x - top_left.x;
        let height = bottom_right.y - top_left.y;

        let mut rect = Self::new(top_left.x, top_left.y, width, height);
        rect.normalize();
        rect.ensure_non_zero_dimension();
        rect
    }

    /// Returns the rect containing all of the supplied points.
    pub fn containing_points(points: Vec<&Point<T>>) -> Option<Self> {
        let x_values: Vec<T> = points.iter().map(|p| p.x).collect();
        let y_values: Vec<T> = points.iter().map(|p| p.y).collect();
        if let (Some(min_x), Some(min_y), Some(max_x), Some(max_y)) = (
            x_values.iter().min(),
            y_values.iter().min(),
            x_values.iter().max(),
            y_values.iter().max(),
        ) {
            let width = *max_x - *min_x + T::one();
            let height = *max_y - *min_y + T::one();

            let mut result = Rect::new(*min_x, *min_y, width, height);
            result.normalize();
            Some(result)
        } else {
            None
        }
    }

    /// Makes sure that the rectangle has a width and height of at least one.
    pub fn ensure_non_zero_dimension(&mut self) {
        self.size.width = T::max(self.size.width, T::one());
        self.size.height = T::max(self.size.height, T::one());
    }

    /// Makes sure that the rectangle has a positive width and height.
    pub fn ensure_positive_dimension(&mut self) {
        self.size.width = T::max(self.size.width, T::zero());
        self.size.height = T::max(self.size.height, T::zero());
    }
}

impl<T> Rect<T>
where
    T: Num + Ord + PrimInt + Signed + AddAssign + From<i32>,
{
    /// Returns a rectangle that contains two floating point points.
    /// The rectangle will fully contain the points.
    /// The rectangle will have a negative width or height if
    /// `point_b` is to the left of or above `point_a`.
    pub fn containing_rounded<U: Num + Float + AddAssign>(
        point_a: &Point<U>,
        point_b: &Point<U>,
    ) -> Rect<T> {
        let min_x = Float::min(point_a.x, point_b.x)
            .floor()
            .to_i32()
            .unwrap()
            .into();
        let max_x: T = Float::max(point_a.x, point_b.x)
            .ceil()
            .to_i32()
            .unwrap()
            .into();
        let min_y = Float::min(point_a.y, point_b.y)
            .floor()
            .to_i32()
            .unwrap()
            .into();
        let max_y: T = Float::max(point_a.y, point_b.y)
            .ceil()
            .to_i32()
            .unwrap()
            .into();
        let mut new_rect = Rect::new(min_x, min_y, max_x - min_x, max_y - min_y);
        if point_b.x < point_a.x {
            new_rect.origin.x += new_rect.size.width;
            new_rect.size.width = T::zero() - new_rect.size.width;
        }
        if point_b.y < point_a.y {
            new_rect.origin.y += new_rect.size.height;
            new_rect.size.height = T::zero() - new_rect.size.height;
        }
        new_rect
    }
}

impl<T> Rect<T>
where
    T: Num + Copy + Ord + Signed + AddAssign,
{
    /// Normalize the rectangle to have a positive width and height.
    pub fn normalize(&mut self) {
        if self.size.width < T::zero() {
            self.origin.x += self.size.width;
            self.size.width = abs(self.size.width)
        }
        if self.size.height < T::zero() {
            self.origin.y += self.size.height;
            self.size.height = abs(self.size.height)
        }
    }
}

impl<T> Rect<T>
where
    T: Num + Float + AddAssign,
{
    /// Creates a rectangle containing two points.
    pub fn containing_float(point_a: Point<T>, point_b: Point<T>) -> Self {
        let mut top_left = Point::zero();
        let mut bottom_right = Point::zero();

        if point_a.x <= point_b.x {
            top_left.x = point_a.x.floor();
            bottom_right.x = point_b.x.ceil();
        } else {
            top_left.x = point_a.x.ceil();
            bottom_right.x = point_b.x.floor();
        }

        if point_a.y <= point_b.y {
            top_left.y = point_a.y.floor();
            bottom_right.y = point_b.y.ceil();
        } else {
            top_left.y = point_a.y.ceil();
            bottom_right.y = point_b.y.floor();
        }

        let width = bottom_right.x - top_left.x;
        let height = bottom_right.y - top_left.y;

        let mut result = Self::new(top_left.x, top_left.y, width, height);
        result.normalize_float();
        result
    }

    /// Returns the frame inset by the edge insets.
    pub fn inset_float(&self, insets: &EdgeInsets<T>) -> Self {
        let x = self.origin.x + insets.left;
        let y = self.origin.y + insets.top;
        let width = self.size.width - insets.left - insets.right;
        let height = self.size.height - insets.top - insets.bottom;
        let mut result = Self::new(x, y, width, height);
        result.normalize_float();
        result
    }

    /// Normalize the rectangle to have a positive width and height.
    pub fn normalize_float(&mut self) {
        if self.size.width < T::zero() {
            self.origin.x += self.size.width;
            self.size.width = Float::abs(self.size.width)
        }
        if self.size.height < T::zero() {
            self.origin.y += self.size.height;
            self.size.height = Float::abs(self.size.height)
        }
    }

    /// Returns the minimum value of the rectangle in the x axis.
    pub fn min_x_float(&self) -> T {
        let right_edge = self.origin.x + self.size.width;
        Float::min(right_edge, self.origin.x)
    }

    /// Returns the maximum value of the rectangle in the x axis.
    pub fn max_x_float(&self) -> T {
        let right_edge = self.origin.x + self.size.width;
        Float::max(right_edge, self.origin.x)
    }

    /// Returns the minimum value of the rectangle in the y axis.
    pub fn min_y_float(&self) -> T {
        let bottom_edge = self.origin.y + self.size.height;
        Float::min(bottom_edge, self.origin.y)
    }

    /// Returns the maximum value of the rectangle in the y axis.
    pub fn max_y_float(&self) -> T {
        let bottom_edge = self.origin.y + self.size.height;
        Float::max(bottom_edge, self.origin.y)
    }

    /// Returns the midpoint of the rectangle on the x axis.
    pub fn mid_x(&self) -> T {
        let width = Float::abs(self.size.width);
        let min_x = self.min_x_float();
        min_x + width * T::from(0.5).unwrap()
    }

    /// Returns the midpoint of the rectangle on the y axis.
    pub fn mid_y(&self) -> T {
        let height = Float::abs(self.size.height);
        let min_y = self.min_y_float();
        min_y + height * T::from(0.5).unwrap()
    }

    /// Returns the midpoint of the rectangle.
    pub fn midpoint(&self) -> Point<T> {
        let mid_x = self.mid_x();
        let mid_y = self.mid_y();
        Point { x: mid_x, y: mid_y }
    }

    /// Returns the rectangle rotated about a point.
    pub fn rotated(&self, angle: T, point: Point<T>) -> Rect<T> {
        let top_left = Point {
            x: self.min_x_float(),
            y: self.min_y_float(),
        };
        let top_right = Point {
            x: self.max_x_float(),
            y: self.min_y_float(),
        };
        let bottom_left = Point {
            x: self.min_x_float(),
            y: self.max_y_float(),
        };
        let bottom_right = Point {
            x: self.max_x_float(),
            y: self.max_y_float(),
        };

        let top_left = top_left.rotated(angle, point);
        let top_right = top_right.rotated(angle, point);
        let bottom_left = bottom_left.rotated(angle, point);
        let bottom_right = bottom_right.rotated(angle, point);

        let x_values = vec![top_left.x, top_right.x, bottom_left.x, bottom_right.x];
        let y_values = vec![top_left.y, top_right.y, bottom_left.y, bottom_right.y];
        let min_x = x_values.iter().fold(Float::infinity(), |a: T, &b| a.min(b));
        let max_x = x_values
            .iter()
            .fold(Float::neg_infinity(), |a: T, &b| a.max(b));
        let min_y = y_values.iter().fold(Float::infinity(), |a: T, &b| a.min(b));
        let max_y = y_values
            .iter()
            .fold(Float::neg_infinity(), |a: T, &b| a.max(b));
        let width = max_x - min_x;
        let height = max_y - min_y;
        Rect::new(min_x, min_y, width, height)
    }
}

impl<T: Float> Rect<T> {
    /// Returns the size rounded and as an integer type.
    pub fn rounded(&self) -> Rect<i32> {
        Rect {
            origin: self.origin.rounded(),
            size: self.size.rounded(),
        }
    }
}

impl<T: Num + Zero> Rect<T> {
    /// Creates a new rectangle with zero origin and size.
    pub fn zero() -> Rect<T> {
        return Rect {
            origin: Point::<T>::zero(),
            size: Size::<T>::zero(),
        };
    }
}

// FROM

impl From<Rect<i32>> for Rect<f32> {
    fn from(value: Rect<i32>) -> Self {
        Self {
            origin: value.origin.into(),
            size: value.size.into(),
        }
    }
}

impl From<Rect<u32>> for Rect<i32> {
    fn from(value: Rect<u32>) -> Self {
        Self {
            origin: value.origin.into(),
            size: value.size.into(),
        }
    }
}

impl From<Rect<u32>> for Rect<f32> {
    fn from(value: Rect<u32>) -> Self {
        Self {
            origin: value.origin.into(),
            size: value.size.into(),
        }
    }
}

impl From<Rect<f32>> for Rect<i32> {
    fn from(value: Rect<f32>) -> Self {
        Self {
            origin: value.origin.into(),
            size: value.size.into(),
        }
    }
}

impl From<Rect<f32>> for Rect<u32> {
    fn from(value: Rect<f32>) -> Self {
        Self {
            origin: value.origin.into(),
            size: value.size.into(),
        }
    }
}

// UTILITIES

impl<T> Rect<T>
where
    T: Num + Ord + Copy + Signed + AddAssign,
{
    /// Returns the minimum value of the rectangle in the x axis.
    pub fn min_x(&self) -> T {
        let right_edge = self.origin.x + self.size.width;
        std::cmp::min(right_edge, self.origin.x)
    }

    /// Returns the maximum value of the rectangle in the x axis.
    pub fn max_x(&self) -> T {
        let right_edge = self.origin.x + self.size.width;
        std::cmp::max(right_edge, self.origin.x)
    }

    /// Returns the minimum value of the rectangle in the y axis.
    pub fn min_y(&self) -> T {
        let bottom_edge = self.origin.y + self.size.height;
        std::cmp::min(bottom_edge, self.origin.y)
    }

    /// Returns the maximum value of the rectangle in the y axis.
    pub fn max_y(&self) -> T {
        let bottom_edge = self.origin.y + self.size.height;
        std::cmp::max(bottom_edge, self.origin.y)
    }

    /// Returns the absolute width.
    pub fn width(&self) -> T {
        self.size.width.abs()
    }

    /// Returns the absolute height.
    pub fn height(&self) -> T {
        self.size.height.abs()
    }

    /// Returns whether or not the point is contained inside the rectangle.
    pub fn contains(&self, point: Point<T>) -> bool {
        point.x >= self.min_x()
            && point.y >= self.min_y()
            && point.x <= self.max_x()
            && point.y <= self.max_y()
    }

    /// Returns the frame inset by the edge insets.
    pub fn inset(&self, insets: &EdgeInsets<T>) -> Self {
        let x = self.origin.x + insets.left;
        let y = self.origin.y + insets.top;
        let width = self.size.width - insets.left - insets.right;
        let height = self.size.height - insets.top - insets.bottom;
        let mut result = Self::new(x, y, width, height);
        result.normalize();
        result
    }

    /// Returns whether or not one rectangle intersects another.
    pub fn intersects(&self, other: &Rect<T>) -> bool {
        self.intersection(other).is_some()
    }

    /// Returns the rectangle that is the interection of this and another rectangle.
    pub fn intersection(&self, other: &Rect<T>) -> Option<Rect<T>> {
        let min_x = std::cmp::max(self.min_x(), other.min_x());
        let max_x = std::cmp::min(self.max_x(), other.max_x());
        let min_y = std::cmp::max(self.min_y(), other.min_y());
        let max_y = std::cmp::min(self.max_y(), other.max_y());

        let width = max_x - min_x;
        let height = max_y - min_y;

        if width < T::zero() || height < T::zero() {
            return None;
        }

        let result = Rect::new(min_x, min_y, width, height);
        Some(result)
    }

    /// Returns a copy of the rect locked to a 1:1 aspect ratio.
    pub fn aspect_locked(&self) -> Self {
        // Work out the smallest dimension and use that for the magnitude
        // of both the width and height.
        let size_dimension = cmp::min(self.width(), self.height());
        let mut width = size_dimension;
        let mut height = size_dimension;
        // Make sure that the signs for width and height are preserved.
        if self.size.width.is_negative() {
            width = width * (T::zero() - T::one());
        }
        if self.size.height.is_negative() {
            height = height * (T::zero() - T::one());
        }
        let new_size = Size { width, height };
        Rect {
            origin: self.origin,
            size: new_size,
        }
    }
}

impl<T> Rect<T>
where
    T: Num + Copy,
{
    /// Returns the point as an array.
    pub fn to_array(&self) -> [[T; 2]; 2] {
        let origin_array = self.origin.to_array();
        let size_array = self.size.to_array();
        [origin_array, size_array]
    }
}

// SERIALISATION

impl<T> Rect<T>
where
    T: Num + Copy + serde::Serialize,
{
    /// Returns the rectangle as a JSON array.
    pub fn to_json_array(&self) -> Result<String, serde_json::Error> {
        let array = self.to_array();
        serde_json::to_string(&array)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_containing() {
        let point_a = Point { x: 9, y: 13 };
        let point_b = Point { x: 9, y: 3 };

        let result = Rect::containing(&point_a, &point_b);

        assert_eq!(result, Rect::new(9, 3, 1, 10));
    }

    #[test]
    fn test_containing_float() {
        let point_a = Point { x: 12.3, y: 14.2 };
        let point_b = Point { x: 2.3, y: 4.1 };

        let result = Rect::containing_float(point_a, point_b);

        assert_eq!(result, Rect::new(2.0, 4.0, 11.0, 11.0));
    }

    #[test]
    fn test_containing_points() {
        let point_a = Point { x: 9, y: 13 };
        let point_b = Point { x: 9, y: 3 };

        let result = Rect::containing_points(vec![&point_a, &point_b]);

        assert_eq!(result, Some(Rect::new(9, 3, 1, 11)));
    }

    #[test]
    fn containing_rounded() {
        let point_a = Point { x: 0.7, y: 1.6 };
        let point_b = Point { x: 3.5, y: 3.7 };

        let expected = Rect::new(0, 1, 4, 3);
        let result: Rect<i32> = Rect::containing_rounded(&point_a, &point_b);
        assert_eq!(result, expected);

        let expected = Rect::new(4, 4, -4, -3);
        let result: Rect<i32> = Rect::containing_rounded(&point_b, &point_a);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_min_x() {
        let rect = Rect::new(3, 4, 10, 23);
        assert_eq!(rect.min_x(), 3);
    }

    #[test]
    fn test_max_x() {
        let rect = Rect::new(3, 4, 10, 23);
        assert_eq!(rect.max_x(), 13);
    }

    #[test]
    fn test_min_y() {
        let rect = Rect::new(3, 4, 10, 23);
        assert_eq!(rect.min_y(), 4);
    }

    #[test]
    fn test_max_y() {
        let rect = Rect::new(3, 4, 10, 23);
        assert_eq!(rect.max_y(), 27);
    }

    #[test]
    fn test_contains() {
        let rect = Rect::new(3, 4, 10, 23);
        let point_inside = Point { x: 7, y: 8 };
        assert!(rect.contains(point_inside));

        let point_outside = Point { x: 2, y: 8 };
        assert!(rect.contains(point_outside) == false);
    }

    #[test]
    fn test_to_json_array() {
        let rect = Rect::new(3, 4, 10, 23);
        let json_string = rect.to_json_array().unwrap();

        let expected_string = "[[3,4],[10,23]]";
        assert_eq!(json_string, expected_string);
    }

    #[test]
    fn test_intersection() {
        let rect_a = Rect::new(0, 0, 6, 6);
        let rect_b = Rect::new(3, 2, 5, 3);
        let expected = Rect::new(3, 2, 3, 3);

        assert_eq!(rect_a.intersection(&rect_b), Some(expected));

        let rect_c = Rect::new(7, 0, 2, 0);
        assert_eq!(rect_a.intersection(&rect_c), None);
        assert_eq!(rect_c.intersection(&rect_a), None);

        assert_eq!(rect_a.intersection(&rect_a), Some(rect_a));
    }

    #[test]
    fn test_inset() {
        let rect = Rect::new(3, 5, 7, 9);
        let insets = EdgeInsets::new(1, 2, 3, 4);

        let new_rect = rect.inset(&insets);

        assert_eq!(new_rect.origin.x, 5);
        assert_eq!(new_rect.origin.y, 6);
        assert_eq!(new_rect.size.width, 1);
        assert_eq!(new_rect.size.height, 5);
    }

    #[test]
    fn test_midpoint() {
        let rect = Rect::new(3.0, 5.0, 7.0, 9.0);
        let midpoint = rect.midpoint();

        assert_eq!(midpoint.x, 6.5);
        assert_eq!(midpoint.y, 9.5);
    }

    #[test]
    fn test_aspect_locked() {
        let rect = Rect::new(10, 10, -5, -7);
        let expected_result = Rect::new(10, 10, -5, -5);
        assert_eq!(rect.aspect_locked(), expected_result);
    }
}
