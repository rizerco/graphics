use serde::Deserialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
#[repr(u32)]
pub enum BlendMode {
    /// Adds color components to achieve a brightening effect.
    Addition = 16,
    /// Uses the luminance values of the background with the hue and saturation values of the source image.
    Color = 14,
    /// Darkens the background image samples to reflect the source image samples.
    ColorBurn = 7,
    /// Brightens the background image samples to reflect the source image samples.
    ColorDodge = 6,
    /// Creates composite image samples by choosing the darker samples (from either the source image or the background).
    Darken = 4,
    /// Subtracts either the source image sample color from the background image sample color, or the reverse,
    /// depending on which sample has the greater brightness value.
    Difference = 10,
    /// Divides the background image sample color from the source image sample color.
    Divide = 18,
    /// Produces an effect similar to that produced by the difference blend mode filter but with lower contrast.
    Exclusion = 11,
    /// Either multiplies or screens colors, depending on the source image sample color.
    HardLight = 8,
    /// Uses the luminance and saturation values of the background image with the hue of the input image.
    Hue = 12,
    /// Creates composite image samples by choosing the lighter samples (either from the source image or the background).
    Lighten = 5,
    /// Uses the hue and saturation of the background image with the luminance of the input image.
    Luminosity = 15,
    /// Multiplies the input image samples with the background image samples.
    Multiply = 1,
    /// Normal blending.
    #[default]
    Normal = 0,
    /// Either multiplies or screens the input image samples with the background image samples, depending on the background color.
    Overlay = 3,
    /// Pass through blending only applies to groups and results in the group’s layers being treated
    /// as if they were part of a flat layer structure.
    PassThrough = 19,
    /// Uses the luminance and hue values of the background image with the saturation of the input image.
    Saturation = 13,
    /// Multiplies the inverse of the input image samples with the inverse of the background image samples.
    Screen = 2,
    /// Either darkens or lightens colors, depending on the input image sample color.
    SoftLight = 9,
    /// Subtracts the background image sample color from the source image sample color.
    Subtract = 17,
    /// Destination which overlaps the source, replaces the source.
    DestinationIn = 20,
    /// Destination is placed, where it falls outside of the source.
    DestinationOut = 21,
}

impl BlendMode {
    // Conversion function from primitive value to enum variant
    pub fn from_primitive(value: u32) -> Option<Self> {
        match value {
            0 => Some(BlendMode::Normal),
            1 => Some(BlendMode::Multiply),
            2 => Some(BlendMode::Screen),
            3 => Some(BlendMode::Overlay),
            4 => Some(BlendMode::Darken),
            5 => Some(BlendMode::Lighten),
            6 => Some(BlendMode::ColorDodge),
            7 => Some(BlendMode::ColorBurn),
            8 => Some(BlendMode::HardLight),
            9 => Some(BlendMode::SoftLight),
            10 => Some(BlendMode::Difference),
            11 => Some(BlendMode::Exclusion),
            12 => Some(BlendMode::Hue),
            13 => Some(BlendMode::Saturation),
            14 => Some(BlendMode::Color),
            15 => Some(BlendMode::Luminosity),
            16 => Some(BlendMode::Addition),
            17 => Some(BlendMode::Subtract),
            18 => Some(BlendMode::Divide),
            19 => Some(BlendMode::PassThrough),
            20 => Some(BlendMode::DestinationIn),
            21 => Some(BlendMode::DestinationOut),
            _ => None,
        }
    }
}

impl BlendMode {
    /// Returns the string representation of the blend mode.
    pub fn as_str(&self) -> &'static str {
        match self {
            BlendMode::Addition => "addition",
            BlendMode::Color => "color",
            BlendMode::ColorBurn => "color-burn",
            BlendMode::ColorDodge => "color-dodge",
            BlendMode::Darken => "darken",
            BlendMode::DestinationIn => "destination-in",
            BlendMode::DestinationOut => "destination-out",
            BlendMode::Difference => "difference",
            BlendMode::Divide => "divide",
            BlendMode::Exclusion => "exclusion",
            BlendMode::HardLight => "hard-light",
            BlendMode::Hue => "hue",
            BlendMode::Lighten => "lighten",
            BlendMode::Luminosity => "luminosity",
            BlendMode::Multiply => "multiply",
            BlendMode::Normal => "normal",
            BlendMode::Overlay => "overlay",
            BlendMode::PassThrough => "pass-through",
            BlendMode::Saturation => "saturation",
            BlendMode::Screen => "screen",
            BlendMode::SoftLight => "soft-light",
            BlendMode::Subtract => "subtract",
        }
    }
}

impl BlendMode {
    /// Creates a blend mode from a string.
    pub fn from_str(string: &str) -> Option<BlendMode> {
        match string {
            "addition" => Some(Self::Addition),
            "color" => Some(Self::Color),
            "colorBurn" | "color_burn" | "color-burn" => Some(Self::ColorBurn),
            "darken" => Some(Self::Darken),
            "destinationIn" | "destination_in" | "destination-in" => Some(Self::DestinationIn),
            "destinationOut" | "destination_out" | "destination-out" => Some(Self::DestinationOut),
            "difference" => Some(Self::Difference),
            "divide" => Some(Self::Divide),
            "exclusion" => Some(Self::Exclusion),
            "hardLight" | "hard_light" | "hard-light" => Some(Self::HardLight),
            "hue" => Some(Self::Hue),
            "lighten" => Some(Self::Lighten),
            "luminosity" => Some(Self::Luminosity),
            "multiply" => Some(Self::Multiply),
            "normal" => Some(Self::Normal),
            "overlay" => Some(Self::Overlay),
            "passThrough" | "pass_trough" | "pass-through" => Some(Self::PassThrough),
            "saturation" => Some(Self::Saturation),
            "screen" => Some(Self::Screen),
            "softLight" | "soft_light" | "soft-light" => Some(Self::SoftLight),
            "subtract" => Some(Self::Subtract),
            _ => None,
        }
    }
}

impl BlendMode {
    /// Returns whether the blend mode is one of the Porter Duff modes.
    pub fn is_porter_duff(&self) -> bool {
        match self {
            BlendMode::DestinationIn | BlendMode::DestinationOut => true,
            _ => false,
        }
    }
}

impl<'de> Deserialize<'de> for BlendMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: Value = Deserialize::deserialize(deserializer)?;
        let key = value
            .as_str()
            .ok_or_else(|| serde::de::Error::custom("Expected a string"))?;
        Self::from_str(key).ok_or(serde::de::Error::custom(
            "Unable to parse a valid blend mode.",
        ))
    }
}
