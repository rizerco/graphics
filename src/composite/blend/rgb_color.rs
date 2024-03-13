use std::ops::{Add, Mul, Sub};

use super::RgbaColor;

/// Defines the colour type used in blend functions.
#[derive(Clone, Debug)]
pub struct RgbColor {
    /// The red component.
    pub red: f32,
    /// The green component.
    pub green: f32,
    /// The blue component.
    pub blue: f32,
}

impl RgbColor {
    /// Creates a new blend colour.
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }
}

// PRESETS

impl RgbColor {
    /// White.
    pub fn white() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
        }
    }
}

// FROM

impl RgbColor {
    /// Creates a new blend colour from the crate colour.
    pub fn from_color(color: &crate::Color) -> Self {
        let max = u8::MAX as f32;
        Self {
            red: color.red as f32 / max,
            green: color.green as f32 / max,
            blue: color.blue as f32 / max,
        }
    }

    /// Creates a new colour from an RGBA colour.
    pub fn from_rgba_color(color: &RgbaColor) -> Self {
        Self {
            red: color.red,
            green: color.green,
            blue: color.blue,
        }
    }

    /// Returns this colour type as a crate colour.
    pub fn to_color(&self) -> crate::Color {
        let max = u8::MAX as f32;
        crate::Color {
            red: (self.red * max).round() as u8,
            green: (self.green * max).round() as u8,
            blue: (self.blue * max).round() as u8,
            alpha: u8::MAX,
        }
    }
}

// ADD

impl Add for RgbColor {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl RgbColor {
    /// Adds the components of two colours together.
    pub fn add(&mut self, other_color: &RgbColor) {
        self.red += other_color.red;
        self.green += other_color.green;
        self.blue += other_color.blue;
    }
}

// SUBTRACT

impl RgbColor {
    /// Subtracts the components of two colours together.
    pub fn subtract(&mut self, other_color: &RgbColor) {
        self.red -= other_color.red;
        self.green -= other_color.green;
        self.blue -= other_color.blue;
    }
}

impl Sub for RgbColor {
    type Output = RgbColor;

    fn sub(self, rhs: Self) -> Self {
        Self {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

// MULTIPLY

impl RgbColor {
    /// Multiplies the components of two colours together.
    pub fn multiply(&mut self, other_color: &RgbColor) {
        self.red *= other_color.red;
        self.green *= other_color.green;
        self.blue *= other_color.blue;
    }
}

impl Mul<RgbColor> for RgbColor {
    type Output = RgbColor;

    fn mul(self, rhs: Self) -> Self {
        Self {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul<f32> for RgbColor {
    type Output = RgbColor;

    fn mul(self, rhs: f32) -> Self {
        Self {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

// MIN

/// Returns a colour that has the minimum components of
/// each colour, much like a shader min function.
pub fn min(color_a: &RgbColor, color_b: &RgbColor) -> RgbColor {
    RgbColor {
        red: f32::min(color_a.red, color_b.red),
        green: f32::min(color_a.green, color_b.green),
        blue: f32::min(color_a.blue, color_b.blue),
    }
}

// MAX

/// Returns a colour that has the maximum components of
/// each colour, much like a shader min function.
pub fn max(color_a: &RgbColor, color_b: &RgbColor) -> RgbColor {
    RgbColor {
        red: f32::max(color_a.red, color_b.red),
        green: f32::max(color_a.green, color_b.green),
        blue: f32::max(color_a.blue, color_b.blue),
    }
}

// CLAMPING

impl RgbColor {
    /// Clamps the colour values to the acceptable range.
    pub fn clamp(&mut self) {
        self.red = self.red.clamp(0.0, 1.0);
        self.green = self.green.clamp(0.0, 1.0);
        self.blue = self.blue.clamp(0.0, 1.0);
    }

    /// Returns the absolute value of all the channels.
    pub fn abs(&mut self) {
        self.red = self.red.abs();
        self.green = self.green.abs();
        self.blue = self.blue.abs();
    }

    /// Rounds all channels.
    pub fn round(&mut self) {
        self.red = self.red.round();
        self.green = self.green.round();
        self.blue = self.blue.round();
    }

    /// Returns a rounded copy.
    pub fn rounded(&mut self) -> RgbColor {
        let mut clone = self.clone();
        clone.round();
        clone
    }
}
