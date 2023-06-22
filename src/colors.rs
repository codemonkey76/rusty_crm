use crossterm::style::Color;

#[derive(Debug,Clone)]
pub struct ColorScheme {
    pub dark_black: Color,
    pub black: Color,
    pub grey: Color,
    pub white: Color,
    pub light_grey: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
}

impl ColorScheme {
    pub fn new() -> ColorScheme {
        ColorScheme {
            dark_black: Color::Rgb { r: 49, g: 51, b: 70 },
            black: Color::Rgb { r: 69, g: 71, b: 90 },
            grey: Color::Rgb { r: 88, g: 91, b: 112 },
            white: Color::Rgb { r: 186, g: 194, b: 222 },
            light_grey: Color::Rgb { r: 166, g: 173, b: 200 },
            red: Color::Rgb { r: 243, g: 139, b: 168 },
            green: Color::Rgb { r: 166, g: 227, b: 161 },
            yellow: Color::Rgb { r: 249, g: 226, b: 175 },
            blue: Color::Rgb { r: 137, g: 180, b: 250 },
            magenta: Color::Rgb { r: 245, g: 194, b: 231 },
            cyan: Color::Rgb { r: 148, g: 226, b: 213 },

        }
    }
}
