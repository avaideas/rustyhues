use crate::{Color, Comparison, Decoration, RGB};

/// Accepted output item types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OutputItem {
    FgColor(Color),
    BgColor(Color),
    FgRgb(RGB),
    BgRgb(RGB),
    Decoration(Decoration),
    Comparison(Comparison, Option<bool>),
}

/// Output
///
/// Struct holding all data necessary for Paint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Output {
    pub(crate) should_reset: bool,
    pub(crate) items: Vec<OutputItem>,
}

impl Output {
    /// Create a new Output.
    pub fn new() -> Self {
        Self {
            should_reset: true,
            items: Vec::new(),
        }
    }

    /// Set `should_reset`
    pub fn set_should_reset(&mut self, reset: bool) {
        self.should_reset = reset;
    }

    /// Push a foreground colour to output.
    pub fn push_fg(&mut self, color: Color) {
        self.items.push(OutputItem::FgColor(color));
    }

    /// Push a background colour to output.
    pub fn push_bg(&mut self, color: Color) {
        self.items.push(OutputItem::BgColor(color));
    }

    /// Push a foreground rgb colour to output.
    pub fn push_fg_rgb(&mut self, rgb: RGB) {
        self.items.push(OutputItem::FgRgb(rgb));
    }

    /// Push a background rgb colour to output.
    pub fn push_bg_rgb(&mut self, rgb: RGB) {
        self.items.push(OutputItem::BgRgb(rgb));
    }

    /// Push a decoration to output.
    pub fn push_decoration(&mut self, decoration: Decoration) {
        self.items.push(OutputItem::Decoration(decoration));
    }

    /// Push a comparison to output
    pub fn push_comparison(&mut self, comparison: Comparison, expression: Option<bool>) {
        self.items
            .push(OutputItem::Comparison(comparison, expression));
    }
}
