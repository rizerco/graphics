mod rgb_color;
mod rgba_color;

pub use rgb_color::RgbColor;
pub use rgba_color::RgbaColor;

/// The sRGB gamma values.
const GAMMA_VALUES: RgbColor = RgbColor {
    red: 0.3,
    green: 0.59,
    blue: 0.11,
};

// HSL CALCULATIONS

// Returns the luminance of a colour.
fn calculate_luminance(color: &RgbColor) -> f32 {
    return GAMMA_VALUES.red * color.red
        + GAMMA_VALUES.green * color.green
        + GAMMA_VALUES.blue * color.blue;
}

// Performs a clip colour operation.
fn clip_color(color: &mut RgbColor) {
    let luminance = calculate_luminance(color);
    let n = f32::min(f32::min(color.red, color.green), color.blue);
    let x = f32::max(f32::max(color.red, color.green), color.blue);
    if n < 0.0 {
        color.red = luminance + (((color.red - luminance) * luminance) / (luminance - n));
        color.green = luminance + (((color.green - luminance) * luminance) / (luminance - n));
        color.blue = luminance + (((color.blue - luminance) * luminance) / (luminance - n));
    }

    if x > 1.0 {
        color.red = luminance + (((color.red - luminance) * (1.0 - luminance)) / (x - luminance));
        color.green =
            luminance + (((color.green - luminance) * (1.0 - luminance)) / (x - luminance));
        color.blue = luminance + (((color.blue - luminance) * (1.0 - luminance)) / (x - luminance));
    }
}

// Sets the luminance of a colour.
fn set_luminance(color: &mut RgbColor, luminance: f32) {
    let delta = luminance - calculate_luminance(color);
    color.red = color.red + delta;
    color.green = color.green + delta;
    color.blue = color.blue + delta;
    clip_color(color);
}

// Returns the saturation of a colour.
fn calculate_saturation(color: &RgbColor) -> f32 {
    return f32::max(f32::max(color.red, color.green), color.blue)
        - f32::min(f32::min(color.red, color.green), color.blue);
}

/// Sets the saturation.
fn set_saturation(color: &mut RgbColor, saturation: f32) {
    let max_value = f32::max(color.red, f32::max(color.green, color.blue));
    let min_value = f32::min(color.red, f32::min(color.green, color.blue));
    let mid_value = (color.red + color.green + color.blue) - (max_value + min_value);

    let new_max: f32;
    let new_mid: f32;

    let rounded_values = (RgbColor::new(min_value, mid_value, max_value) * 255.0).rounded();
    let rounded_max = rounded_values.blue;
    let rounded_mid = rounded_values.green;
    let rounded_color = (color.clone() * 255.0).rounded();

    color.red = rounded_color.red;
    color.green = rounded_color.green;
    color.blue = rounded_color.blue;

    if max_value > min_value {
        new_mid = ((mid_value - min_value) * saturation) / (max_value - min_value);
        new_max = saturation;
    } else {
        new_mid = 0.0;
        new_max = 0.0;
    }
    let new_min: f32 = 0.0;

    // Set the red
    if rounded_color.red == rounded_max {
        color.red = new_max;
    } else if rounded_color.red == rounded_mid {
        color.red = new_mid;
    } else {
        color.red = new_min;
    }

    // Set the green
    if rounded_color.green == rounded_max {
        color.green = new_max;
    } else if rounded_color.green == rounded_mid {
        color.green = new_mid;
    } else {
        color.green = new_min;
    }

    // Set the blue
    if rounded_color.blue == rounded_max {
        color.blue = new_max;
    } else if rounded_color.blue == rounded_mid {
        color.blue = new_mid;
    } else {
        color.blue = new_min;
    }
}

// ADDITION

/// Calculate the addition blend mode.
pub fn addition(color: &mut RgbColor, blend: &RgbColor) {
    color.add(blend);
    color.clamp();
}

// COLOUR

/// Calculate color.
pub fn color(color: &mut RgbColor, blend: &RgbColor) {
    let luminance = calculate_luminance(color);
    let mut result = blend.clone();
    set_luminance(&mut result, luminance);
    color.red = result.red;
    color.green = result.green;
    color.blue = result.blue;
}

// COLOUR BURN

/// Calculate the colour burn for a value.
fn color_burn_value(base: f32, blend: f32) -> f32 {
    if base == 1.0 {
        return base;
    } else if blend == 0.0 {
        return blend;
    } else {
        return f32::max(1.0 - ((1.0 - base) / blend), 0.0);
    }
}

/// Calculate the colour burn.
pub fn color_burn(color: &mut RgbColor, blend: &RgbColor) {
    color.red = color_burn_value(color.red, blend.red);
    color.green = color_burn_value(color.green, blend.green);
    color.blue = color_burn_value(color.blue, blend.blue);
}

// COLOUR DODGE

/// Calculate the colour dodge for a value.
fn color_dodge_value(base: f32, blend: f32) -> f32 {
    if blend >= 1.0 {
        return 1.0;
    } else {
        return f32::min(base / (1.0 - blend), 1.0);
    };
}

/// Calculate the colour dodge for a colour.
pub fn color_dodge(color: &mut RgbColor, blend: &RgbColor) {
    color.red = color_dodge_value(color.red, blend.red);
    color.green = color_dodge_value(color.green, blend.green);
    color.blue = color_dodge_value(color.blue, blend.blue);
}

// DARKEN

/// Calculate the darken for a value.
fn darken_value(base: f32, blend: f32) -> f32 {
    return f32::min(blend, base);
}

/// Calculate the darken for a colour.
pub fn darken(color: &mut RgbColor, blend: &RgbColor) {
    color.red = darken_value(color.red, blend.red);
    color.green = darken_value(color.green, blend.green);
    color.blue = darken_value(color.blue, blend.blue);
}

// DESTINATION IN

/// Caluculate the destination in blend mode.
pub fn destination_in(color: &mut RgbaColor, blend: &RgbaColor, opacity: f32) {
    color.alpha *= blend.alpha * opacity;
}

// DESTINATION OUT

/// Caluculate the destination out blend mode.
pub fn destination_out(color: &mut RgbaColor, blend: &RgbaColor, opacity: f32) {
    color.alpha *= opacity * (1.0 - blend.alpha);
}

// DIFFERENCE

/// Calculate the difference for a colour.
pub fn difference(color: &mut RgbColor, blend: &RgbColor) {
    color.subtract(blend);
    color.abs();
}

// DIVIDE

/// Calculate the divide for a value.
fn divide_value(base: f32, blend: f32) -> f32 {
    if blend == 1.0 {
        return blend;
    } else if base == 0.0 {
        return 0.0;
    } else {
        return f32::min(base / blend, 1.0);
    }
}

/// Calculate the divide for a colour.
pub fn divide(color: &mut RgbColor, blend: &RgbColor) {
    color.red = divide_value(color.red, blend.red);
    color.green = divide_value(color.green, blend.green);
    color.blue = divide_value(color.blue, blend.blue);
}

// EXCLSUION

/// Calculate the exclusion for a colour.
pub fn exclusion(color: &mut RgbColor, blend: &RgbColor) {
    let base = color.clone();
    let blend = blend.clone();
    let result = base.clone() + blend.clone() - base * blend * 2.0;
    color.red = result.red;
    color.green = result.green;
    color.blue = result.blue;
}

// HARD LIGHT

/// Calculate the hard light for a colour.
pub fn hard_light(color: &mut RgbColor, blend: &RgbColor) {
    color.red = overlay_value(blend.red, color.red);
    color.green = overlay_value(blend.green, color.green);
    color.blue = overlay_value(blend.blue, color.blue);
}

// // HUE

/// Calculate hue.
pub fn hue(color: &mut RgbColor, blend: &RgbColor) {
    let luminance = calculate_luminance(color);
    let saturation = calculate_saturation(color);
    let mut clone = blend.clone();
    set_saturation(&mut clone, saturation);
    set_luminance(&mut clone, luminance);
    color.red = clone.red;
    color.green = clone.green;
    color.blue = clone.blue;
}

// LIGHTEN

/// Calculate the lighten for a value.
fn lighten_value(base: f32, blend: f32) -> f32 {
    return f32::max(blend, base);
}

/// Calculate the lighten for a colour.
pub fn lighten(color: &mut RgbColor, blend: &RgbColor) {
    color.red = lighten_value(color.red, blend.red);
    color.green = lighten_value(color.green, blend.green);
    color.blue = lighten_value(color.blue, blend.blue);
}

// LUMINOSITY

/// Calculate luminosity.
pub fn luminosity(color: &mut RgbColor, blend: &RgbColor) {
    let luminance = calculate_luminance(blend);
    set_luminance(color, luminance);
}

// MULTIPLY

/// Calculate the multiply for a colour.
pub fn multiply(color: &mut RgbColor, blend: &RgbColor) {
    color.multiply(blend);
}

// OVERLAY

/// Calculate the overlay for a value.
fn overlay_value(base: f32, blend: f32) -> f32 {
    if base < 0.5 {
        return 2.0 * base * blend;
    } else {
        return 1.0 - 2.0 * (1.0 - base) * (1.0 - blend);
    };
}

/// Calculate the overlay for a colour.
pub fn overlay(color: &mut RgbColor, blend: &RgbColor) {
    color.red = overlay_value(color.red, blend.red);
    color.green = overlay_value(color.green, blend.green);
    color.blue = overlay_value(color.blue, blend.blue);
}

// SATURATION

/// Calculate saturation.
pub fn saturation(color: &mut RgbColor, blend: &RgbColor) {
    let luminance = calculate_luminance(color);
    let saturation = calculate_saturation(blend);
    set_saturation(color, saturation);
    set_luminance(color, luminance);
}

// SCREEN

/// Calculate the screen for a value.
fn screen_value(base: f32, blend: f32) -> f32 {
    return 1.0 - (1.0 - base) * (1.0 - blend);
}

/// Calculate the screen for a colour.
pub fn screen(color: &mut RgbColor, blend: &RgbColor) {
    color.red = screen_value(color.red, blend.red);
    color.green = screen_value(color.green, blend.green);
    color.blue = screen_value(color.blue, blend.blue);
}

// SOFT LIGHT

/// Calculate the soft light component used in the W3C spec.
fn soft_light_component(base: f32) -> f32 {
    if base <= 0.25 {
        return ((16.0 * base - 12.0) * base + 4.0) * base;
    } else {
        return base.sqrt();
    }
}

/// Calculate the soft light for a value.
fn soft_light_value(base: f32, blend: f32) -> f32 {
    // Pegtop formula.
    // return (1.0 - 2.0 * blend) * base * base + 2.0 * blend * base;

    // Original Photoshop formula.
    // if blend < 0.5 {
    //     return 2.0 * base * blend + base * base * (1.0 - 2.0 * blend);
    // } else {
    //     return sqrt(base) * (2.0 * blend - 1.0) + 2.0 * base * (1.0 - blend);
    // }

    // W3C spec formula.
    if blend <= 0.5 {
        return base - (1.0 - 2.0 * blend) * base * (1.0 - base);
    } else {
        return base + (2.0 * blend - 1.0) * (soft_light_component(base) - base);
    }
}

/// Calculate the soft light for a colour.
pub fn soft_light(color: &mut RgbColor, blend: &RgbColor) {
    color.red = soft_light_value(color.red, blend.red);
    color.green = soft_light_value(color.green, blend.green);
    color.blue = soft_light_value(color.blue, blend.blue);
}

// SUBTRACT

/// Calculate the subtract for a colour.
pub fn subtract(color: &mut RgbColor, blend: &RgbColor) {
    color.subtract(blend);
    color.clamp();
}
