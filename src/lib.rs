//! RustyHues — lightweight ANSI color styling for Rust.
//!
//! ```rust
//! use rustyhues::*;
//! println!("{}", "Hello".blue().bold());
//! ```
pub mod env;
mod output;
mod paint;
mod stream;
mod style;
#[cfg(windows)]
mod windows;

use env::{predefined_style, PredefinedStyleType};
pub use paint::Paint;
pub use style::{
    Color, Comparison, Decoration, COLOR_BG_BLACK_CODE, COLOR_BG_BLUE_CODE,
    COLOR_BG_BRIGHT_BLACK_CODE, COLOR_BG_BRIGHT_BLUE_CODE, COLOR_BG_BRIGHT_CYAN_CODE,
    COLOR_BG_BRIGHT_GREEN_CODE, COLOR_BG_BRIGHT_MAGENTA_CODE, COLOR_BG_BRIGHT_RED_CODE,
    COLOR_BG_BRIGHT_WHITE_CODE, COLOR_BG_BRIGHT_YELLOW_CODE, COLOR_BG_CYAN_CODE,
    COLOR_BG_GREEN_CODE, COLOR_BG_MAGENTA_CODE, COLOR_BG_RED_CODE, COLOR_BG_RGB,
    COLOR_BG_WHITE_CODE, COLOR_BG_YELLOW_CODE, COLOR_BLACK_CODE, COLOR_BLUE_CODE,
    COLOR_BRIGHT_BLACK_CODE, COLOR_BRIGHT_BLUE_CODE, COLOR_BRIGHT_CYAN_CODE,
    COLOR_BRIGHT_GREEN_CODE, COLOR_BRIGHT_MAGENTA_CODE, COLOR_BRIGHT_RED_CODE,
    COLOR_BRIGHT_WHITE_CODE, COLOR_BRIGHT_YELLOW_CODE, COLOR_CYAN_CODE, COLOR_GREEN_CODE,
    COLOR_MAGENTA_CODE, COLOR_RED_CODE, COLOR_RGB, COLOR_WHITE_CODE, COLOR_YELLOW_CODE,
    DECORATION_BOLD_CODE, DECORATION_DIM_CODE, DECORATION_INVERT_CODE, DECORATION_ITALIC_CODE,
    DECORATION_UNDERLINE_CODE, RGB,
};

/// Stylize trait.
///
/// Create new `Paint` contexts and apply styles to them.
pub trait Stylize: Sized {
    /// Create a new `Paint` context that writes to std_out.
    fn paint(self) -> Paint<Self> {
        Paint::new_stdout(self)
    }

    /// Create a new `Paint` context that writes to std_err.
    fn paint_err(self) -> Paint<Self> {
        Paint::new_stderr(self)
    }

    /// Set should reset
    fn should_reset(self, reset: bool) -> Paint<Self> {
        self.paint().should_reset(reset)
    }

    /// Set no reset
    fn no_reset(self) -> Paint<Self> {
        self.paint().no_reset()
    }

    /// Create a new std_out `Paint` context and colour content using an RGB value.
    fn rgb(self, r: u8, g: u8, b: u8) -> Paint<Self> {
        self.paint().set_rgb(RGB::new(r, g, b))
    }

    /// Create a new std_out `Paint` context and colour the background of content using an RGB value.
    fn bg_rgb(self, r: u8, g: u8, b: u8) -> Paint<Self> {
        self.paint().set_bg_rgb(RGB::new(r, g, b))
    }

    /// Create a new std_out `Paint` context and colour content using a Hex string.
    /// For example: FFFFFF
    fn hex(self, hex: &str) -> Paint<Self> {
        if let Some(rgb) = RGB::new_from_hex(hex) {
            let updated_self = self.paint().set_rgb(rgb);
            updated_self
        } else {
            self.paint()
        }
    }

    /// Create a new std_out `Paint` context and colour the background of content using a Hex string.
    /// For example: 333333
    fn bg_hex(self, hex: &str) -> Paint<Self> {
        if let Some(rgb) = RGB::new_from_hex(hex) {
            let updated_self = self.paint().set_bg_rgb(rgb);
            updated_self
        } else {
            self.paint()
        }
    }

    /// Create a new std_out `Paint` context and colour content ANSI Black.
    fn black(self) -> Paint<Self> {
        self.paint().fg(Color::Black)
    }

    /// Create a new std_out `Paint` context and colour content ANSI Red.
    fn red(self) -> Paint<Self> {
        self.paint().fg(Color::Red)
    }

    /// Create a new std_out `Paint` context and colour content ANSI Green.
    fn green(self) -> Paint<Self> {
        self.paint().fg(Color::Green)
    }

    /// Create a new std_out `Paint` context and colour content ANSI Yellow.
    fn yellow(self) -> Paint<Self> {
        self.paint().fg(Color::Yellow)
    }

    /// Create a new std_out `Paint` context and colour content ANSI Blue.
    fn blue(self) -> Paint<Self> {
        self.paint().fg(Color::Blue)
    }

    /// Create a new std_out `Paint` context and colour content ANSI Magenta.
    fn magenta(self) -> Paint<Self> {
        self.paint().fg(Color::Magenta)
    }

    /// Create a new std_out `Paint` context and colour content ANSI Cyan.
    fn cyan(self) -> Paint<Self> {
        self.paint().fg(Color::Cyan)
    }

    /// Create a new std_out `Paint` context and colour content ANSI White.
    fn white(self) -> Paint<Self> {
        self.paint().fg(Color::White)
    }

    /// Create a new std_out `Paint` context and colour content ANSI Bright Black.
    fn bright_black(self) -> Paint<Self> {
        self.paint().fg(Color::BrightBlack)
    }

    /// Create a new std_out `Paint` context and colour content ANSI Bright Red.
    fn bright_red(self) -> Paint<Self> {
        self.paint().fg(Color::BrightRed)
    }

    /// Create a new std_out `Paint` context and colour content ANSI Bright Green.
    fn bright_green(self) -> Paint<Self> {
        self.paint().fg(Color::BrightGreen)
    }

    /// Create a new std_out `Paint` context and colour content ANSI Bright Yellow.
    fn bright_yellow(self) -> Paint<Self> {
        self.paint().fg(Color::BrightYellow)
    }

    /// Create a new std_out `Paint` context and colour content ANSI Bright Blue.
    fn bright_blue(self) -> Paint<Self> {
        self.paint().fg(Color::BrightBlue)
    }

    /// Create a new std_out `Paint` context and colour content ANSI Bright Magenta.
    fn bright_magenta(self) -> Paint<Self> {
        self.paint().fg(Color::BrightMagenta)
    }

    /// Create a new std_out `Paint` context and colour content ANSI Bright Cyan.
    fn bright_cyan(self) -> Paint<Self> {
        self.paint().fg(Color::BrightCyan)
    }

    /// Create a new std_out `Paint` context and colour content ANSI Bright White.
    fn bright_white(self) -> Paint<Self> {
        self.paint().fg(Color::BrightWhite)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Black.
    fn bg_black(self) -> Paint<Self> {
        self.paint().bg(Color::Black)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Red.
    fn bg_red(self) -> Paint<Self> {
        self.paint().bg(Color::Red)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Green.
    fn bg_green(self) -> Paint<Self> {
        self.paint().bg(Color::Green)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Yellow.
    fn bg_yellow(self) -> Paint<Self> {
        self.paint().bg(Color::Yellow)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Blue.
    fn bg_blue(self) -> Paint<Self> {
        self.paint().bg(Color::Blue)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Magenta.
    fn bg_magenta(self) -> Paint<Self> {
        self.paint().bg(Color::Magenta)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Cyan.
    fn bg_cyan(self) -> Paint<Self> {
        self.paint().bg(Color::Cyan)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI White.
    fn bg_white(self) -> Paint<Self> {
        self.paint().bg(Color::White)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Bright Black.
    fn bg_bright_black(self) -> Paint<Self> {
        self.paint().bg(Color::BrightBlack)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Bright Red.
    fn bg_bright_red(self) -> Paint<Self> {
        self.paint().bg(Color::BrightRed)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Bright Green.
    fn bg_bright_green(self) -> Paint<Self> {
        self.paint().bg(Color::BrightGreen)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Bright Yellow.
    fn bg_bright_yellow(self) -> Paint<Self> {
        self.paint().bg(Color::BrightYellow)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Bright Blue.
    fn bg_bright_blue(self) -> Paint<Self> {
        self.paint().bg(Color::BrightBlue)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Bright Magenta.
    fn bg_bright_magenta(self) -> Paint<Self> {
        self.paint().bg(Color::BrightMagenta)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Bright Cyan.
    fn bg_bright_cyan(self) -> Paint<Self> {
        self.paint().bg(Color::BrightCyan)
    }

    /// Create a new std_out `Paint` context and colour the background of content ANSI Bright White.
    fn bg_bright_white(self) -> Paint<Self> {
        self.paint().bg(Color::BrightWhite)
    }

    /// Create a new std_out `Paint` context and decorate content ANSI Bold.
    fn bold(self) -> Paint<Self> {
        self.paint().decoration(Decoration::Bold)
    }

    /// Create a new std_out `Paint` context and decorate content ANSI Dim.
    fn dim(self) -> Paint<Self> {
        self.paint().decoration(Decoration::Dim)
    }

    /// Create a new std_out `Paint` context and decorate content ANSI Italic.
    fn italic(self) -> Paint<Self> {
        self.paint().decoration(Decoration::Italic)
    }

    /// Create a new std_out `Paint` context and decorate content ANSI Underline.
    fn underline(self) -> Paint<Self> {
        self.paint().decoration(Decoration::Underline)
    }

    /// Create a new std_out `Paint` context and decorate content ANSI Invert.
    fn invert(self) -> Paint<Self> {
        self.paint().decoration(Decoration::Invert)
    }

    /// Set an is comparison with or without an expression
    fn is(self, expression: Option<bool>) -> Paint<Self> {
        self.paint().set_is_not(Comparison::Is, expression)
    }

    /// Set a not comparison with or without an expression
    fn not(self, expression: Option<bool>) -> Paint<Self> {
        self.paint().set_is_not(Comparison::Not, expression)
    }

    /// Set an is comparison with an expression (expression is)
    fn e_is(self, expression: bool) -> Paint<Self> {
        self.paint().set_is_not(Comparison::Is, Some(expression))
    }

    /// Set a not comparison with an expression (expression not)
    fn e_not(self, expression: bool) -> Paint<Self> {
        self.paint().set_is_not(Comparison::Not, Some(expression))
    }

    /// Set an is comparison with no expression (none is)
    fn n_is(self) -> Paint<Self> {
        self.paint().set_is_not(Comparison::Is, None)
    }

    /// Set a not comparison with no expression (none not)
    fn n_not(self) -> Paint<Self> {
        self.paint().set_is_not(Comparison::Not, None)
    }

    /// Success style shortcut
    fn success(self) -> Paint<Self> {
        let mut paint = self.paint();
        let styles = predefined_style(PredefinedStyleType::Success);
        if let Some(fg) = styles.fg {
            paint = paint.fg(fg);
        }
        if let Some(bg) = styles.bg {
            paint = paint.bg(bg);
        }
        if let Some(decoration) = styles.decoration {
            paint = paint.decoration(decoration);
        }
        paint
    }

    /// Error style shortcut
    fn error(self) -> Paint<Self> {
        let mut paint = self.paint();
        let styles = predefined_style(PredefinedStyleType::Error);
        if let Some(fg) = styles.fg {
            paint = paint.fg(fg);
        }
        if let Some(bg) = styles.bg {
            paint = paint.bg(bg);
        }
        if let Some(decoration) = styles.decoration {
            paint = paint.decoration(decoration);
        }
        paint
    }

    /// Warning style shortcut
    fn warning(self) -> Paint<Self> {
        let mut paint = self.paint();
        let styles = predefined_style(PredefinedStyleType::Warning);
        if let Some(fg) = styles.fg {
            paint = paint.fg(fg);
        }
        if let Some(bg) = styles.bg {
            paint = paint.bg(bg);
        }
        if let Some(decoration) = styles.decoration {
            paint = paint.decoration(decoration);
        }
        paint
    }

    /// Info style shortcut
    fn info(self) -> Paint<Self> {
        let mut paint = self.paint();
        let styles = predefined_style(PredefinedStyleType::Info);
        if let Some(fg) = styles.fg {
            paint = paint.fg(fg);
        }
        if let Some(bg) = styles.bg {
            paint = paint.bg(bg);
        }
        if let Some(decoration) = styles.decoration {
            paint = paint.decoration(decoration);
        }
        paint
    }

    /// Debug style shortcut
    fn debug(self) -> Paint<Self> {
        let mut paint = self.paint();
        let styles = predefined_style(PredefinedStyleType::Debug);
        if let Some(fg) = styles.fg {
            paint = paint.fg(fg);
        }
        if let Some(bg) = styles.bg {
            paint = paint.bg(bg);
        }
        if let Some(decoration) = styles.decoration {
            paint = paint.decoration(decoration);
        }
        paint
    }
}

impl<T> Stylize for T {}
