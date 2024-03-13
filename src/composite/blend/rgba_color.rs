use std::ops::{Add, Mul};

use super::RgbColor;

/// Defines the colour type used in blend functions.
#[derive(Debug)]
pub struct RgbaColor {
    /// The red component.
    pub red: f32,
    /// The green component.
    pub green: f32,
    /// The blue component.
    pub blue: f32,
    /// The alpha component.
    pub alpha: f32,
}

// PREMULTIPLY

impl RgbaColor {
    /// Premultiplies the colour channels by the alpha channel.
    pub fn premultiply(&mut self) {
        if self.alpha < 1.0 && self.alpha > 0.0 {
            self.red *= self.alpha;
            self.green *= self.alpha;
            self.blue *= self.alpha;
        }
    }

    /// Unpremultiplies the colour channels by the alpha channel.
    pub fn unpremultiply(&mut self) {
        if self.alpha < 1.0 && self.alpha > 0.0 {
            self.red /= self.alpha;
            self.green /= self.alpha;
            self.blue /= self.alpha;
        }
    }
}

// FROM

impl From<RgbColor> for RgbaColor {
    fn from(color: RgbColor) -> Self {
        Self {
            red: color.red,
            green: color.green,
            blue: color.blue,
            alpha: 1.0,
        }
    }
}

impl RgbaColor {
    /// Creates a new blend colour from the crate colour.
    pub fn from(color: &crate::Color) -> Self {
        let max = u8::MAX as f32;
        Self {
            red: color.red as f32 / max,
            green: color.green as f32 / max,
            blue: color.blue as f32 / max,
            alpha: color.alpha as f32 / max,
        }
    }

    /// Returns this colour type as a crate colour.
    pub fn to_color(&self) -> crate::Color {
        let max = u8::MAX as f32;
        crate::Color {
            red: (self.red * max).round() as u8,
            green: (self.green * max).round() as u8,
            blue: (self.blue * max).round() as u8,
            alpha: (self.alpha * max).round() as u8,
        }
    }
}

// MATHS

impl Mul<f32> for RgbaColor {
    type Output = RgbaColor;

    fn mul(self, rhs: f32) -> Self {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
            alpha: self.alpha * rhs,
        }
    }
}

impl Add for RgbaColor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
            alpha: self.alpha + rhs.alpha,
        }
    }
}
