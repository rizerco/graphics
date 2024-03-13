use num_traits::Float;

/// Defines a colour in the RGBA format.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Color {
    /// The red component.
    pub red: u8,
    /// The green component.
    pub green: u8,
    /// The blue component.
    pub blue: u8,
    /// The alpha component.
    pub alpha: u8,
}

impl Default for Color {
    fn default() -> Self {
        Color::BLACK
    }
}

impl Color {
    /// Creates a new colour from a hex value in the RGB format.
    pub fn from_rgb_u32(value: u32) -> Color {
        let red = (value & 0xff0000) >> 16;
        let green = (value & 0x00ff00) >> 8;
        let blue = value & 0x0000ff;
        let alpha = 0xff;
        Color {
            red: red.try_into().unwrap(),
            green: green.try_into().unwrap(),
            blue: blue.try_into().unwrap(),
            alpha,
        }
    }

    /// Creates a new colour from a hex value in the RGBA format.
    pub fn from_rgba_u32(value: u32) -> Color {
        let red = (value & 0xff000000) >> 24;
        let green = (value & 0x00ff0000) >> 16;
        let blue = (value & 0x0000ff00) >> 8;
        let alpha = value & 0x000000ff;
        Color {
            red: red.try_into().unwrap(),
            green: green.try_into().unwrap(),
            blue: blue.try_into().unwrap(),
            alpha: alpha.try_into().unwrap(),
        }
    }

    /// Creates a new colour from a hex value in the ARGB format.
    pub fn from_argb_u32(value: u32) -> Color {
        let alpha = (value & 0xff000000) >> 24;
        let red = (value & 0x00ff0000) >> 16;
        let green = (value & 0x0000ff00) >> 8;
        let blue = value & 0x000000ff;
        Color {
            red: red.try_into().unwrap(),
            green: green.try_into().unwrap(),
            blue: blue.try_into().unwrap(),
            alpha: alpha.try_into().unwrap(),
        }
    }

    /// Returns the colour as an unsigned integer in an
    /// RGB format.
    pub fn as_rgb_u32(&self) -> u32 {
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | (self.blue as u32)
    }

    /// Returns the colour as an unsigned integer in an
    /// RGBA format.
    pub fn as_rgba_u32(&self) -> u32 {
        ((self.red as u32) << 24)
            | ((self.green as u32) << 16)
            | ((self.blue as u32) << 8)
            | (self.alpha as u32)
    }

    /// Returns the colour as an unsigned integer in an
    /// ARGB format.
    pub fn as_argb_u32(&self) -> u32 {
        ((self.alpha as u32) << 24)
            | ((self.red as u32) << 16)
            | ((self.green as u32) << 8)
            | (self.blue as u32)
    }

    /// Returns the colour as a hex string.
    /// Discards any alpha value.
    pub fn as_hex(&self, include_octothorpe: bool) -> String {
        let mut result = format!("{:x}", self.as_rgb_u32());
        if include_octothorpe {
            result.insert(0, '#');
        }
        result
    }

    /// Creates a colour from HSB values, each provided in the
    /// range between 0 and 1.
    pub fn from_hsb(hue: f32, saturation: f32, brightness: f32) -> Self {
        let k = |n: f32| (n + hue * 6.0) % 6.0;
        let f = |n: f32| {
            brightness
                * (1.0 - saturation * f32::max(0.0, f32::min(k(n), f32::min(4.0 - k(n), 1.0))))
        };

        let red = (255.0 * f(5.0)).round() as u8;
        let green = (255.0 * f(3.0)).round() as u8;
        let blue = (255.0 * f(1.0)).round() as u8;

        Self {
            red,
            green,
            blue,
            alpha: 0xff,
        }
    }

    /// Creates a colour from HSB and alpha values, each provided in the
    /// range between 0 and 1.
    pub fn from_hsba(hue: f32, saturation: f32, brightness: f32, alpha: f32) -> Self {
        let k = |n: f32| (n + hue * 6.0) % 6.0;
        let f = |n: f32| {
            brightness
                * (1.0 - saturation * f32::max(0.0, f32::min(k(n), f32::min(4.0 - k(n), 1.0))))
        };

        let red = (255.0 * f(5.0)).round() as u8;
        let green = (255.0 * f(3.0)).round() as u8;
        let blue = (255.0 * f(1.0)).round() as u8;
        let alpha = (255.0 * alpha).round() as u8;

        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Returns the hue of a colour, in the range 0 to 1.
    pub fn hue(&self) -> f32 {
        let values = vec![self.red, self.green, self.blue];
        let (Some(min), Some(max)) = (values.iter().min(), values.iter().max()) else {
            return 0.0;
        };

        if min == max {
            return 0.0;
        }

        let denominator = (max - min) as f32;

        let red = self.red as f32;
        let green = self.green as f32;
        let blue = self.blue as f32;

        let mut hue = if max == &self.red {
            (green - blue) as f32 / denominator
        } else if max == &self.green {
            2.0 + (blue - red) as f32 / denominator
        } else {
            4.0 + (red - green) as f32 / denominator
        };

        hue /= 6.0;

        if hue < 0.0 {
            hue += 1.0;
        }
        hue
    }

    /// Modifies the hue of the colour.
    pub fn set_hue(&mut self, hue: f32) {
        let saturation = self.saturation();
        let brightness = self.brightness();
        let new_color = Color::from_hsb(hue, saturation, brightness);
        self.red = new_color.red;
        self.green = new_color.green;
        self.blue = new_color.blue;
    }

    /// Returns the saturation of a colour, in the range 0 to 1.
    pub fn saturation(&self) -> f32 {
        let values = vec![self.red, self.green, self.blue];
        let (Some(min), Some(max)) = (values.iter().min(), values.iter().max()) else {
            return 0.0;
        };

        if max == &0 {
            return 0.0;
        }

        (max - min) as f32 / *max as f32
    }

    /// Modifies the saturation of the colour.
    pub fn set_saturation(&mut self, saturation: f32) {
        let hue = self.hue();
        let brightness = self.brightness();
        let new_color = Color::from_hsb(hue, saturation, brightness);
        self.red = new_color.red;
        self.green = new_color.green;
        self.blue = new_color.blue;
    }

    /// Returns the brightness of a colour, in the range 0 to 1.
    pub fn brightness(&self) -> f32 {
        let values = vec![self.red, self.green, self.blue];
        let Some(max) = values.iter().max() else {
            return 0.0;
        };
        *max as f32 / u8::MAX as f32
    }

    /// Modifies the brightness of the colour.
    pub fn set_brightness(&mut self, brightness: f32) {
        let hue = self.hue();
        let saturation = self.saturation();
        let new_color = Color::from_hsb(hue, saturation, brightness);
        self.red = new_color.red;
        self.green = new_color.green;
        self.blue = new_color.blue;
    }
}

// INTO

impl From<[u8; 4]> for Color {
    fn from(array: [u8; 4]) -> Self {
        Self {
            red: array[0],
            green: array[1],
            blue: array[2],
            alpha: array[3],
        }
    }
}

impl From<Color> for [u8; 4] {
    fn from(value: Color) -> Self {
        [value.red, value.green, value.blue, value.alpha]
    }
}

impl From<&Color> for [u8; 4] {
    fn from(value: &Color) -> Self {
        [value.red, value.green, value.blue, value.alpha]
    }
}

// PRESETS

impl Color {
    /// Returns a white colour.
    pub const WHITE: Color = {
        Color {
            red: 0xff,
            green: 0xff,
            blue: 0xff,
            alpha: 0xff,
        }
    };

    /// Returns a black colour.
    pub const BLACK: Color = {
        Color {
            red: 0x00,
            green: 0x00,
            blue: 0x00,
            alpha: 0xff,
        }
    };

    /// Returns a transparent colour.
    pub const CLEAR: Color = {
        Color {
            red: 0x00,
            green: 0x00,
            blue: 0x00,
            alpha: 0x00,
        }
    };

    /// Returns a red colour.
    pub const RED: Color = {
        Color {
            red: 0xff,
            green: 0x00,
            blue: 0x00,
            alpha: 0xff,
        }
    };

    /// Returns a yellow colour.
    pub const YELLOW: Color = {
        Color {
            red: 0xff,
            green: 0xff,
            blue: 0x00,
            alpha: 0xff,
        }
    };

    /// Returns a green colour.
    pub const GREEN: Color = {
        Color {
            red: 0x00,
            green: 0xff,
            blue: 0x00,
            alpha: 0xff,
        }
    };

    /// Returns a cyan colour.
    pub const CYAN: Color = {
        Color {
            red: 0x00,
            green: 0xff,
            blue: 0xff,
            alpha: 0xff,
        }
    };

    /// Returns a blue colour.
    pub const BLUE: Color = {
        Color {
            red: 0x00,
            green: 0x00,
            blue: 0xff,
            alpha: 0xff,
        }
    };

    /// Returns a magenta colour.
    pub const MAGENTA: Color = {
        Color {
            red: 0xff,
            green: 0x00,
            blue: 0xff,
            alpha: 0xff,
        }
    };
}

// RANDOM

impl Color {
    /// Returns a random colour.
    pub fn random() -> Self {
        let red = rand::random::<u8>();
        let green = rand::random::<u8>();
        let blue = rand::random::<u8>();
        Self {
            red,
            green,
            blue,
            alpha: 0xff,
        }
    }
}

// MARK: Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_rgb_u32() {
        let value: u32 = 0xe4a672;
        let color = Color::from_rgb_u32(value);

        assert_eq!(color.red, 0xe4);
        assert_eq!(color.green, 0xa6);
        assert_eq!(color.blue, 0x72);
        assert_eq!(color.alpha, 0xff);
    }

    #[test]
    fn test_from_rgba_u32() {
        let value: u32 = 0xe4a672ff;
        let color = Color::from_rgba_u32(value);

        assert_eq!(color.red, 0xe4);
        assert_eq!(color.green, 0xa6);
        assert_eq!(color.blue, 0x72);
        assert_eq!(color.alpha, 0xff);
    }

    #[test]
    fn test_from_argb_u32() {
        let value: u32 = 0xffe4a672;
        let color = Color::from_argb_u32(value);

        assert_eq!(color.red, 0xe4);
        assert_eq!(color.green, 0xa6);
        assert_eq!(color.blue, 0x72);
        assert_eq!(color.alpha, 0xff);
    }

    #[test]
    fn test_from_hsb() {
        let color = Color::from_hsb(0.21588946146, 0.7975206611570248, 0.9490196078431372);

        assert_eq!(color.red, 0xb9);
        assert_eq!(color.green, 0xf2);
        assert_eq!(color.blue, 0x31);
        assert_eq!(color.alpha, 0xff);
    }

    #[test]
    fn hue() {
        let color = Color::RED;
        assert_eq!(color.hue(), 0.0);
        let color = Color::YELLOW;
        assert_eq!(color.hue(), 1.0 / 6.0);
        let color = Color::GREEN;
        assert_eq!(color.hue(), 2.0 / 6.0);
        let color = Color::CYAN;
        assert_eq!(color.hue(), 3.0 / 6.0);
        let color = Color::BLUE;
        assert_eq!(color.hue(), 4.0 / 6.0);
        let color = Color::MAGENTA;
        assert_eq!(color.hue(), 5.0 / 6.0);
        let color = Color::from_rgb_u32(0xB1D2FF);
        assert_eq!(color.hue(), 0.59615386);
    }

    #[test]
    fn set_hue() {
        let mut color = Color::from_rgb_u32(0xB1D2FF);
        color.set_hue(130.0 / 360.0);
        let expected_color = Color::from_rgb_u32(0xB1FFBE);
        assert_eq!(color, expected_color);
    }

    #[test]
    fn saturation() {
        let color = Color::from_rgb_u32(0xB1D2FF);
        assert_eq!(color.saturation(), 0.30588236);
    }

    #[test]
    fn set_saturation() {
        let mut color = Color::from_rgb_u32(0xB1D2FF);
        color.set_saturation(0.7);
        let expected_color = Color::from_rgb_u32(0x4D98FF);
        assert_eq!(color, expected_color);
    }

    #[test]
    fn brightness() {
        let color = Color::from_rgb_u32(0xD57540);
        assert_eq!(color.brightness(), 0.8352941);
    }

    #[test]
    fn set_brightness() {
        let mut color = Color::from_rgb_u32(0xB1D2FF);
        color.set_brightness(0.63);
        let expected_color = Color::from_rgb_u32(0x7084A1);
        assert_eq!(color, expected_color);
    }

    #[test]
    fn test_as_hex() {
        let value: u32 = 0xe4a672;
        let color = Color::from_rgb_u32(value);
        assert_eq!(color.as_hex(false), "e4a672".to_string());
        assert_eq!(color.as_hex(true), "#e4a672".to_string());
    }
}
