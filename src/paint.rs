use crate::{
    env::{predefined_style, should_colorize, true_color_allowed, PredefinedStyleType},
    output::{Output, OutputItem},
    stream,
    style::{Color, Comparison, Decoration, RGB},
};
use core::fmt;

pub struct Paint<T> {
    pub(crate) inner: T,
    pub(crate) enabled: bool,
    pub(crate) output: Output,
}

/// Paint
///
/// Paint context for managing and displaying styles, including methods for chaining styles together.
impl<T> Paint<T> {
    /// Initialize a new Paint context for writing to std_out.
    fn new(inner: T, enabled: bool) -> Self {
        Self {
            inner,
            enabled,
            output: Output::new(),
        }
    }

    /// Initialize a new Paint context for writing to std_out explicitly.
    pub fn new_stdout(inner: T) -> Self {
        let enabled = should_colorize(stream::Stream::Stdout);
        Self::new(inner, enabled)
    }

    /// Initialize a new Paint context for writing to std_err explicitly.
    pub fn new_stderr(inner: T) -> Self {
        let enabled = should_colorize(stream::Stream::Stderr);
        Self::new(inner, enabled)
    }

    /// Set the foreground colour of the content.
    pub fn fg(mut self, color: Color) -> Self {
        if self.enabled {
            self.output.push_fg(color);
        }
        self
    }

    /// Set the background colour of the content.
    pub fn bg(mut self, color: Color) -> Self {
        if self.enabled {
            self.output.push_bg(color);
        }
        self
    }

    /// Set the decorations to be used on the content.
    pub fn decoration(mut self, decoration: Decoration) -> Self {
        if self.enabled {
            self.output.push_decoration(decoration);
        }
        self
    }

    /// Set an is or not expression
    pub fn set_is_not(mut self, comparison: Comparison, expression: Option<bool>) -> Self {
        if self.enabled {
            self.output.push_comparison(comparison, expression);
        }
        self
    }

    /// Set the foreground RGB value for the content.
    pub fn set_rgb(mut self, rgb_value: RGB) -> Self {
        if self.enabled {
            self.output.push_fg_rgb(rgb_value);
        }
        self
    }

    /// Set the background RGB value for the content.
    pub fn set_bg_rgb(mut self, rgb_value: RGB) -> Self {
        if self.enabled {
            self.output.push_bg_rgb(rgb_value);
        }
        self
    }

    /// Set should reset
    pub fn should_reset(mut self, reset: bool) -> Self {
        self.output.set_should_reset(reset);
        self
    }

    /// Set no reset
    pub fn no_reset(mut self) -> Self {
        self.output.set_should_reset(false);
        self
    }

    /// Add a RGB colour to the content (For use when chaining styles).
    pub fn rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.set_rgb(RGB::new(r, g, b))
    }

    /// Add a background RGB colour to the content (For use when chaining styles).
    pub fn bg_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.set_bg_rgb(RGB::new(r, g, b))
    }

    /// Add a Hex colour to the content (For use when chaining styles).
    pub fn hex(self, hex: &str) -> Self {
        if let Some(rgb) = RGB::new_from_hex(hex) {
            self.set_rgb(rgb)
        } else {
            self
        }
    }

    /// Add a background Hex colour to the content (For use when chaining styles).
    pub fn bg_hex(self, hex: &str) -> Self {
        if let Some(rgb) = RGB::new_from_hex(hex) {
            self.set_bg_rgb(rgb)
        } else {
            self
        }
    }

    /// Add ANSI Black colour to the content (For use when chaining styles).
    pub fn black(self) -> Self {
        self.fg(Color::Black)
    }

    /// Add ANSI Red colour to the content (For use when chaining styles).
    pub fn red(self) -> Self {
        self.fg(Color::Red)
    }

    /// Add ANSI Green colour to the content (For use when chaining styles).
    pub fn green(self) -> Self {
        self.fg(Color::Green)
    }

    /// Add ANSI Yellow colour to the content (For use when chaining styles).
    pub fn yellow(self) -> Self {
        self.fg(Color::Yellow)
    }

    /// Add ANSI Blue colour to the content (For use when chaining styles).
    pub fn blue(self) -> Self {
        self.fg(Color::Blue)
    }

    /// Add ANSI Magenta colour to the content (For use when chaining styles).
    pub fn magenta(self) -> Self {
        self.fg(Color::Magenta)
    }

    /// Add ANSI Cyan colour to the content (For use when chaining styles).
    pub fn cyan(self) -> Self {
        self.fg(Color::Cyan)
    }

    /// Add ANSI White colour to the content (For use when chaining styles).
    pub fn white(self) -> Self {
        self.fg(Color::White)
    }

    /// Add ANSI Bright Black colour to the content (For use when chaining styles).
    pub fn bright_black(self) -> Self {
        self.fg(Color::BrightBlack)
    }

    /// Add ANSI Bright Red colour to the content (For use when chaining styles).
    pub fn bright_red(self) -> Self {
        self.fg(Color::BrightRed)
    }

    /// Add ANSI Bright Green colour to the content (For use when chaining styles).
    pub fn bright_green(self) -> Self {
        self.fg(Color::BrightGreen)
    }

    /// Add ANSI Bright Yellow colour to the content (For use when chaining styles).
    pub fn bright_yellow(self) -> Self {
        self.fg(Color::BrightYellow)
    }

    /// Add ANSI Bright Blue colour to the content (For use when chaining styles).
    pub fn bright_blue(self) -> Self {
        self.fg(Color::BrightBlue)
    }

    /// Add ANSI Bright Magenta colour to the content (For use when chaining styles).
    pub fn bright_magenta(self) -> Self {
        self.fg(Color::BrightMagenta)
    }

    /// Add ANSI Bright Cyan colour to the content (For use when chaining styles).
    pub fn bright_cyan(self) -> Self {
        self.fg(Color::BrightCyan)
    }

    /// Add ANSI Bright White colour to the content (For use when chaining styles).
    pub fn bright_white(self) -> Self {
        self.fg(Color::BrightWhite)
    }

    /// Add ANSI Black background colour to the content (For use when chaining styles).
    pub fn bg_black(self) -> Self {
        self.bg(Color::Black)
    }

    /// Add ANSI Red background colour to the content (For use when chaining styles).
    pub fn bg_red(self) -> Self {
        self.bg(Color::Red)
    }

    /// Add ANSI Green background colour to the content (For use when chaining styles).
    pub fn bg_green(self) -> Self {
        self.bg(Color::Green)
    }

    /// Add ANSI Yellow background colour to the content (For use when chaining styles).
    pub fn bg_yellow(self) -> Self {
        self.bg(Color::Yellow)
    }

    /// Add ANSI Blue background colour to the content (For use when chaining styles).
    pub fn bg_blue(self) -> Self {
        self.bg(Color::Blue)
    }

    // Add ANSI Magenta background colour to the content (For use when chaining styles).
    pub fn bg_magenta(self) -> Self {
        self.bg(Color::Magenta)
    }

    /// Add ANSI Cyan background colour to the content (For use when chaining styles).
    pub fn bg_cyan(self) -> Self {
        self.bg(Color::Cyan)
    }

    /// Add ANSI White background colour to the content (For use when chaining styles).
    pub fn bg_white(self) -> Self {
        self.bg(Color::White)
    }

    /// Add ANSI Bright Black background colour to the content (For use when chaining styles).
    pub fn bg_bright_black(self) -> Self {
        self.bg(Color::BrightBlack)
    }

    /// Add ANSI Bright Red background colour to the content (For use when chaining styles).
    pub fn bg_bright_red(self) -> Self {
        self.bg(Color::BrightRed)
    }

    /// Add ANSI Bright Green background colour to the content (For use when chaining styles).
    pub fn bg_bright_green(self) -> Self {
        self.bg(Color::BrightGreen)
    }

    /// Add ANSI Bright Yellow background colour to the content (For use when chaining styles).
    pub fn bg_bright_yellow(self) -> Self {
        self.bg(Color::BrightYellow)
    }

    /// Add ANSI Bright Blue background colour to the content (For use when chaining styles).
    pub fn bg_bright_blue(self) -> Self {
        self.bg(Color::BrightBlue)
    }

    /// Add ANSI Bright Magenta background colour to the content (For use when chaining styles).
    pub fn bg_bright_magenta(self) -> Self {
        self.bg(Color::BrightMagenta)
    }

    /// Add ANSI Bright Cyan background colour to the content (For use when chaining styles).
    pub fn bg_bright_cyan(self) -> Self {
        self.bg(Color::BrightCyan)
    }

    /// Add ANSI Bright White background colour to the content (For use when chaining styles).
    pub fn bg_bright_white(self) -> Self {
        self.bg(Color::BrightWhite)
    }

    /// Add ANSI Bold decoration to the content (For use when chaining styles).
    pub fn bold(self) -> Self {
        self.decoration(Decoration::Bold)
    }

    /// Add ANSI Dim decoration to the content (For use when chaining styles).
    pub fn dim(self) -> Self {
        self.decoration(Decoration::Dim)
    }

    /// Add ANSI Italic decoration to the content (For use when chaining styles).
    pub fn italic(self) -> Self {
        self.decoration(Decoration::Italic)
    }

    /// Add ANSI Underline decoration to the content (For use when chaining styles).
    pub fn underline(self) -> Self {
        self.decoration(Decoration::Underline)
    }

    /// Add ANSI Invert decoration to the content (For use when chaining styles).
    pub fn invert(self) -> Self {
        self.decoration(Decoration::Invert)
    }

    /// Set an is comparison with an optional expression (For use when chaining styles).
    pub fn is(self, expression: Option<bool>) -> Self {
        self.set_is_not(Comparison::Is, expression)
    }

    /// Set a not comparison with an optional expression (For use when chaining styles).
    pub fn not(self, expression: Option<bool>) -> Self {
        self.set_is_not(Comparison::Not, expression)
    }

    /// Set an is comparison with an expression (expression is) (For use when chaining styles).
    pub fn e_is(self, expression: bool) -> Self {
        self.set_is_not(Comparison::Is, Some(expression))
    }

    /// Set a not comparison with an expression (expression not) (For use when chaining styles).
    pub fn e_not(self, expression: bool) -> Self {
        self.set_is_not(Comparison::Not, Some(expression))
    }

    /// Set an is comparison with no expression (none is) (For use when chaining styles).
    pub fn n_is(self) -> Self {
        self.set_is_not(Comparison::Is, None)
    }

    /// Set a not comparison with no expression (none not) (For use when chaining styles).
    pub fn n_not(self) -> Self {
        self.set_is_not(Comparison::Not, None)
    }

    /// Success style shortcut (For use when chaining styles).
    pub fn success(self) -> Self {
        let mut new_self = self;
        let styles = predefined_style(PredefinedStyleType::Success);
        if let Some(fg) = styles.fg {
            new_self = new_self.fg(fg);
        }
        if let Some(bg) = styles.bg {
            new_self = new_self.bg(bg);
        }
        if let Some(decoration) = styles.decoration {
            new_self = new_self.decoration(decoration);
        }
        new_self
    }

    /// Error style shortcut (For use when chaining styles).
    pub fn error(self) -> Self {
        let mut new_self = self;
        let styles = predefined_style(PredefinedStyleType::Error);
        if let Some(fg) = styles.fg {
            new_self = new_self.fg(fg);
        }
        if let Some(bg) = styles.bg {
            new_self = new_self.bg(bg);
        }
        if let Some(decoration) = styles.decoration {
            new_self = new_self.decoration(decoration);
        }
        new_self
    }

    /// Warning style shortcut (For use when chaining styles).
    pub fn warning(self) -> Self {
        let mut new_self = self;
        let styles = predefined_style(PredefinedStyleType::Warning);
        if let Some(fg) = styles.fg {
            new_self = new_self.fg(fg);
        }
        if let Some(bg) = styles.bg {
            new_self = new_self.bg(bg);
        }
        if let Some(decoration) = styles.decoration {
            new_self = new_self.decoration(decoration);
        }
        new_self
    }

    /// Info style shortcut (For use when chaining styles).
    pub fn info(self) -> Self {
        let mut new_self = self;
        let styles = predefined_style(PredefinedStyleType::Info);
        if let Some(fg) = styles.fg {
            new_self = new_self.fg(fg);
        }
        if let Some(bg) = styles.bg {
            new_self = new_self.bg(bg);
        }
        if let Some(decoration) = styles.decoration {
            new_self = new_self.decoration(decoration);
        }
        new_self
    }

    /// Debug style shortcut (For use when chaining styles).
    pub fn debug(self) -> Self {
        let mut new_self = self;
        let styles = predefined_style(PredefinedStyleType::Debug);
        if let Some(fg) = styles.fg {
            new_self = new_self.fg(fg);
        }
        if let Some(bg) = styles.bg {
            new_self = new_self.bg(bg);
        }
        if let Some(decoration) = styles.decoration {
            new_self = new_self.decoration(decoration);
        }
        new_self
    }
}

impl<T: fmt::Display> fmt::Display for Paint<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.enabled {
            return self.inner.fmt(f);
        }

        if !self.output.items.is_empty() {
            f.write_str("\x1b[")?;
            let mut first = true;
            let mut should_paint = true;

            for item in &self.output.items {
                match item {
                    OutputItem::FgColor(color) => {
                        if should_paint {
                            push_raw(f, &mut first, |f| f.write_str(color.fg_code()))?;
                        }
                    }
                    OutputItem::BgColor(color) => {
                        if should_paint {
                            push_raw(f, &mut first, |f| f.write_str(color.bg_code()))?;
                        }
                    }
                    OutputItem::FgRgb(rgb) => {
                        if should_paint {
                            if true_color_allowed() {
                                push_raw(f, &mut first, |f| rgb.write_fg_sgr(f))?;
                            } else if let Some(color) = rgb.closest_color() {
                                push_raw(f, &mut first, |f| f.write_str(color.fg_code()))?;
                            }
                        }
                    }
                    OutputItem::BgRgb(rgb) => {
                        if should_paint {
                            if true_color_allowed() {
                                push_raw(f, &mut first, |f| rgb.write_bg_sgr(f))?;
                            } else if let Some(color) = rgb.closest_color() {
                                push_raw(f, &mut first, |f| f.write_str(color.bg_code()))?;
                            }
                        }
                    }
                    OutputItem::Decoration(decoration) => {
                        if should_paint {
                            push_raw(f, &mut first, |f| f.write_str(decoration.code()))?;
                        }
                    }
                    OutputItem::Comparison(comparison, expression) => {
                        match (comparison, expression) {
                            (Comparison::Is, Some(true)) => should_paint = true,
                            (Comparison::Is, Some(false)) => should_paint = false,
                            (Comparison::Not, Some(true)) => should_paint = false,
                            (Comparison::Not, Some(false)) => should_paint = true,
                            (_, None) => should_paint = !should_paint,
                        }
                    }
                }
            }

            f.write_str("m")?;
        }

        self.inner.fmt(f)?;

        if !self.output.items.is_empty() && self.output.should_reset {
            f.write_str("\x1b[0m")?;
        }

        Ok(())
    }
}

fn push_raw<F>(f: &mut fmt::Formatter<'_>, first: &mut bool, mut write_param: F) -> fmt::Result
where
    F: FnMut(&mut fmt::Formatter<'_>) -> fmt::Result,
{
    if !*first {
        f.write_str(";")?;
    }
    *first = false;
    write_param(f)
}
