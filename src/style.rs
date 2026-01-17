use core::fmt;

pub const COLOR_BLACK_CODE: &str = "30";
pub const COLOR_RED_CODE: &str = "31";
pub const COLOR_GREEN_CODE: &str = "32";
pub const COLOR_YELLOW_CODE: &str = "33";
pub const COLOR_BLUE_CODE: &str = "34";
pub const COLOR_MAGENTA_CODE: &str = "35";
pub const COLOR_CYAN_CODE: &str = "36";
pub const COLOR_WHITE_CODE: &str = "37";
pub const COLOR_BRIGHT_BLACK_CODE: &str = "90";
pub const COLOR_BRIGHT_RED_CODE: &str = "91";
pub const COLOR_BRIGHT_GREEN_CODE: &str = "92";
pub const COLOR_BRIGHT_YELLOW_CODE: &str = "93";
pub const COLOR_BRIGHT_BLUE_CODE: &str = "94";
pub const COLOR_BRIGHT_MAGENTA_CODE: &str = "95";
pub const COLOR_BRIGHT_CYAN_CODE: &str = "96";
pub const COLOR_BRIGHT_WHITE_CODE: &str = "97";

pub const COLOR_BG_BLACK_CODE: &str = "40";
pub const COLOR_BG_RED_CODE: &str = "41";
pub const COLOR_BG_GREEN_CODE: &str = "42";
pub const COLOR_BG_YELLOW_CODE: &str = "43";
pub const COLOR_BG_BLUE_CODE: &str = "44";
pub const COLOR_BG_MAGENTA_CODE: &str = "45";
pub const COLOR_BG_CYAN_CODE: &str = "46";
pub const COLOR_BG_WHITE_CODE: &str = "47";
pub const COLOR_BG_BRIGHT_BLACK_CODE: &str = "100";
pub const COLOR_BG_BRIGHT_RED_CODE: &str = "101";
pub const COLOR_BG_BRIGHT_GREEN_CODE: &str = "102";
pub const COLOR_BG_BRIGHT_YELLOW_CODE: &str = "103";
pub const COLOR_BG_BRIGHT_BLUE_CODE: &str = "104";
pub const COLOR_BG_BRIGHT_MAGENTA_CODE: &str = "105";
pub const COLOR_BG_BRIGHT_CYAN_CODE: &str = "106";
pub const COLOR_BG_BRIGHT_WHITE_CODE: &str = "107";

pub const COLOR_RGB: &str = "38;2;";
pub const COLOR_BG_RGB: &str = "48;2;";

pub const DECORATION_BOLD_CODE: &str = "1";
pub const DECORATION_DIM_CODE: &str = "2";
pub const DECORATION_ITALIC_CODE: &str = "3";
pub const DECORATION_UNDERLINE_CODE: &str = "4";
pub const DECORATION_INVERT_CODE: &str = "7";

const RGB_COLORS: [((u8, u8, u8), Color); 16] = [
    ((0, 0, 0), Color::Black),
    ((205, 49, 49), Color::Red),
    ((13, 188, 121), Color::Green),
    ((229, 229, 16), Color::Yellow),
    ((36, 114, 200), Color::Blue),
    ((188, 63, 188), Color::Magenta),
    ((17, 168, 205), Color::Cyan),
    ((229, 229, 229), Color::White),
    ((102, 102, 102), Color::BrightBlack),
    ((241, 76, 76), Color::BrightRed),
    ((35, 209, 139), Color::BrightGreen),
    ((245, 245, 67), Color::BrightYellow),
    ((59, 142, 234), Color::BrightBlue),
    ((214, 112, 214), Color::BrightMagenta),
    ((41, 184, 219), Color::BrightCyan),
    ((255, 255, 255), Color::BrightWhite),
];

/// Color
///
/// ANSI colours with methods to get foreground and background codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    /// Get foreground code for a particular ANSI colour.
    pub const fn fg_code(self) -> &'static str {
        match self {
            Color::Black => COLOR_BLACK_CODE,
            Color::Red => COLOR_RED_CODE,
            Color::Green => COLOR_GREEN_CODE,
            Color::Yellow => COLOR_YELLOW_CODE,
            Color::Blue => COLOR_BLUE_CODE,
            Color::Magenta => COLOR_MAGENTA_CODE,
            Color::Cyan => COLOR_CYAN_CODE,
            Color::White => COLOR_WHITE_CODE,
            Color::BrightBlack => COLOR_BRIGHT_BLACK_CODE,
            Color::BrightRed => COLOR_BRIGHT_RED_CODE,
            Color::BrightGreen => COLOR_BRIGHT_GREEN_CODE,
            Color::BrightYellow => COLOR_BRIGHT_YELLOW_CODE,
            Color::BrightBlue => COLOR_BRIGHT_BLUE_CODE,
            Color::BrightMagenta => COLOR_BRIGHT_MAGENTA_CODE,
            Color::BrightCyan => COLOR_BRIGHT_CYAN_CODE,
            Color::BrightWhite => COLOR_BRIGHT_WHITE_CODE,
        }
    }

    /// Get background code for a particular ANSI colour.
    pub const fn bg_code(self) -> &'static str {
        match self {
            Color::Black => COLOR_BG_BLACK_CODE,
            Color::Red => COLOR_BG_RED_CODE,
            Color::Green => COLOR_BG_GREEN_CODE,
            Color::Yellow => COLOR_BG_YELLOW_CODE,
            Color::Blue => COLOR_BG_BLUE_CODE,
            Color::Magenta => COLOR_BG_MAGENTA_CODE,
            Color::Cyan => COLOR_BG_CYAN_CODE,
            Color::White => COLOR_BG_WHITE_CODE,
            Color::BrightBlack => COLOR_BG_BRIGHT_BLACK_CODE,
            Color::BrightRed => COLOR_BG_BRIGHT_RED_CODE,
            Color::BrightGreen => COLOR_BG_BRIGHT_GREEN_CODE,
            Color::BrightYellow => COLOR_BG_BRIGHT_YELLOW_CODE,
            Color::BrightBlue => COLOR_BG_BRIGHT_BLUE_CODE,
            Color::BrightMagenta => COLOR_BG_BRIGHT_MAGENTA_CODE,
            Color::BrightCyan => COLOR_BG_BRIGHT_CYAN_CODE,
            Color::BrightWhite => COLOR_BG_BRIGHT_WHITE_CODE,
        }
    }
}

/// Decoration
///
/// ANSI decorations with methods to get codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Decoration {
    Bold,
    Dim,
    Italic,
    Underline,
    Invert,
}

impl Decoration {
    /// Get code for a particular ANSI decoration.
    pub const fn code(self) -> &'static str {
        match self {
            Decoration::Bold => DECORATION_BOLD_CODE,
            Decoration::Dim => DECORATION_DIM_CODE,
            Decoration::Italic => DECORATION_ITALIC_CODE,
            Decoration::Underline => DECORATION_UNDERLINE_CODE,
            Decoration::Invert => DECORATION_INVERT_CODE,
        }
    }
}

/// RGB
///
/// Struct containing methods for RGB and Hex values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RGB {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

impl RGB {
    /// Create a new RGB value.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Create a new RGB value from a 3 or 6 character hex string.
    /// If Hex is invalid then this will gracefully fail and not add a style.
    pub fn new_from_hex(hex: &str) -> Option<Self> {
        let hex_str = validate_hex_value(hex)?;

        if hex_str.len() == 6 {
            let r = u8::from_str_radix(&hex_str[0..2], 16).unwrap();
            let g = u8::from_str_radix(&hex_str[2..4], 16).unwrap();
            let b = u8::from_str_radix(&hex_str[4..6], 16).unwrap();
            Some(Self { r, g, b })
        } else {
            let mut hex_chars = hex_str.chars();
            let r_char = hex_chars.next().unwrap();
            let g_char = hex_chars.next().unwrap();
            let b_char = hex_chars.next().unwrap();
            let r = u8::from_str_radix(&format!("{r_char}{r_char}"), 16).unwrap();
            let g = u8::from_str_radix(&format!("{g_char}{g_char}"), 16).unwrap();
            let b = u8::from_str_radix(&format!("{b_char}{b_char}"), 16).unwrap();
            Some(Self { r, g, b })
        }
    }

    /// Write a foreground RGB value to a formatter.
    pub fn write_fg_sgr(self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(COLOR_RGB)?;
        write!(f, "{};{};{}", self.r, self.g, self.b)
    }

    /// Write a background RGB value to a formatter.
    pub fn write_bg_sgr(self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(COLOR_BG_RGB)?;
        write!(f, "{};{};{}", self.r, self.g, self.b)
    }

    /// Work out the closest ANSI colour to an RGB value for use if terminal doesn't support RGB.
    pub fn closest_color(self) -> Option<Color> {
        let mut closest_color: Option<Color> = None;
        let mut closest_distance: i32 = i32::MAX;

        for (rgb, color) in RGB_COLORS {
            let (r, g, b) = rgb;
            let dr = (self.r as i32 - r as i32).pow(2);
            let dg = (self.g as i32 - g as i32).pow(2);
            let db = (self.b as i32 - b as i32).pow(2);
            let d = dr + dg + db;
            if d < closest_distance {
                closest_color = Some(color);
                closest_distance = d;
            }
        }

        closest_color
    }
}

fn validate_hex_value(hex: &str) -> Option<&str> {
    let hex_str = hex.strip_prefix("#").unwrap_or(hex);

    if hex_str.len() != 6 && hex_str.len() != 3 {
        return None;
    }

    if !hex_str.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }

    Some(hex_str)
}

/// Comparison
///
/// Valid Comparisons
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comparison {
    Is,
    Not,
}
