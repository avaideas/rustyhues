use crate::stream::Stream;
use crate::style::{Color, Decoration};
#[cfg(windows)]
use crate::windows;
use std::env;
use std::sync::RwLock;

/// COLOR_CHOICE
///
/// Choose whether output should always be styled, never be styled, or auto select whether to style or not.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorChoice {
    Auto,
    Always,
    Never,
}
static COLOR_CHOICE: RwLock<ColorChoice> = RwLock::new(ColorChoice::Auto);
/// Get the current `COLOR_CHOICE` setting.
pub fn color_choice() -> ColorChoice {
    *COLOR_CHOICE.read().unwrap()
}
/// Set a new `COLOR_CHOICE` setting.
pub fn set_color_choice(choice: ColorChoice) {
    if color_choice() != choice {
        *COLOR_CHOICE.write().unwrap() = choice;
    }
}

/// TRUE_COLOR_ALLOWED
///
/// Choose whether RGB should be allowed or not.
/// TRUE_COLOR_ALLOWED is automatically set using terminal settings, it can be overridden using `set_true_color_allowed()`.
static TRUE_COLOR_ALLOWED: RwLock<bool> = RwLock::new(true);
/// Get the current `TRUE_COLOR_ALLOWED` setting.
pub fn true_color_allowed() -> bool {
    *TRUE_COLOR_ALLOWED.read().unwrap()
}
/// Set a new `TRUE_COLOR_ALLOWED` setting.
pub fn set_true_color_allowed(override_allowed: Option<bool>) {
    let mut is_allowed = matches!(env::var("COLORTERM").as_deref(), Ok("truecolor" | "24bit"));

    if let Some(override_val) = override_allowed {
        is_allowed = override_val;
    }

    *TRUE_COLOR_ALLOWED.write().unwrap() = is_allowed;
    set_true_color_initialized();
}

// Store whether `TRUE_COLOR_ALLOWED` has been initialized.
static TRUE_COLOR_INITIALIZED: RwLock<bool> = RwLock::new(false);
fn true_color_initialized() -> bool {
    *TRUE_COLOR_INITIALIZED.read().unwrap()
}
fn set_true_color_initialized() {
    *TRUE_COLOR_INITIALIZED.write().unwrap() = true;
}

/// Work out whether output should be styled or not based on terminal settings.
pub fn should_colorize(stream: Stream) -> bool {
    if !true_color_initialized() {
        set_true_color_allowed(None);
    }

    if color_choice() == ColorChoice::Never {
        return false;
    }
    if color_choice() == ColorChoice::Always {
        return true;
    }

    if env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if matches!(env::var("TERM").as_deref(), Ok("dumb")) {
        return false;
    }

    if matches!(env::var("CLICOLOR").as_deref(), Ok("0")) {
        return false;
    }
    if matches!(env::var("CLICOLOR_FORCE").as_deref(), Ok("1")) {
        return true;
    }

    match stream {
        Stream::Stdout => is_tty_stdout(),
        Stream::Stderr => is_tty_stderr(),
    }
}

#[cfg(unix)]
fn is_tty_stdout() -> bool {
    is_tty_fd(1)
}
#[cfg(unix)]
fn is_tty_stderr() -> bool {
    is_tty_fd(2)
}

#[cfg(unix)]
fn is_tty_fd(fd: i32) -> bool {
    unsafe { libc::isatty(fd) == 1 }
}

#[cfg(windows)]
fn is_tty_stdout() -> bool {
    windows::is_tty_stdout()
}
#[cfg(windows)]
fn is_tty_stderr() -> bool {
    windows::is_tty_stderr()
}

/// Predefined Styles
///
/// Choose the styles for predefined success, warning, error, info, and debug styles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PredefinedStyleType {
    Success,
    Warning,
    Error,
    Info,
    Debug,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PredefinedStyle {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub decoration: Option<Decoration>,
}
static DEFAULT_SUCCESS_STYLE: PredefinedStyle = PredefinedStyle {
    fg: Some(Color::Green),
    bg: None,
    decoration: Some(Decoration::Bold),
};
static DEFAULT_WARNING_STYLE: PredefinedStyle = PredefinedStyle {
    fg: Some(Color::Yellow),
    bg: None,
    decoration: Some(Decoration::Bold),
};
static DEFAULT_ERROR_STYLE: PredefinedStyle = PredefinedStyle {
    fg: Some(Color::Red),
    bg: None,
    decoration: Some(Decoration::Bold),
};
static DEFAULT_INFO_STYLE: PredefinedStyle = PredefinedStyle {
    fg: Some(Color::Blue),
    bg: None,
    decoration: None,
};
static DEFAULT_DEBUG_STYLE: PredefinedStyle = PredefinedStyle {
    fg: Some(Color::Magenta),
    bg: None,
    decoration: None,
};
static SUCCESS_STYLE: RwLock<PredefinedStyle> = RwLock::new(DEFAULT_SUCCESS_STYLE);
static WARNING_STYLE: RwLock<PredefinedStyle> = RwLock::new(DEFAULT_WARNING_STYLE);
static ERROR_STYLE: RwLock<PredefinedStyle> = RwLock::new(DEFAULT_ERROR_STYLE);
static INFO_STYLE: RwLock<PredefinedStyle> = RwLock::new(DEFAULT_INFO_STYLE);
static DEBUG_STYLE: RwLock<PredefinedStyle> = RwLock::new(DEFAULT_DEBUG_STYLE);

/// Get the styles for a predefined style type
pub fn predefined_style(style_type: PredefinedStyleType) -> PredefinedStyle {
    match style_type {
        PredefinedStyleType::Success => *SUCCESS_STYLE.read().unwrap(),
        PredefinedStyleType::Warning => *WARNING_STYLE.read().unwrap(),
        PredefinedStyleType::Error => *ERROR_STYLE.read().unwrap(),
        PredefinedStyleType::Info => *INFO_STYLE.read().unwrap(),
        PredefinedStyleType::Debug => *DEBUG_STYLE.read().unwrap(),
    }
}

/// Set the styles for a predefined style type
pub fn set_predefined_style(
    style_type: PredefinedStyleType,
    fg: Option<Color>,
    bg: Option<Color>,
    decoration: Option<Decoration>,
) {
    match style_type {
        PredefinedStyleType::Success => {
            *SUCCESS_STYLE.write().unwrap() = PredefinedStyle { fg, bg, decoration }
        }
        PredefinedStyleType::Warning => {
            *WARNING_STYLE.write().unwrap() = PredefinedStyle { fg, bg, decoration }
        }
        PredefinedStyleType::Error => {
            *ERROR_STYLE.write().unwrap() = PredefinedStyle { fg, bg, decoration }
        }
        PredefinedStyleType::Info => {
            *INFO_STYLE.write().unwrap() = PredefinedStyle { fg, bg, decoration }
        }
        PredefinedStyleType::Debug => {
            *DEBUG_STYLE.write().unwrap() = PredefinedStyle { fg, bg, decoration }
        }
    }
}

/// Set the styles for the predefined success style type
pub fn set_success_style(fg: Option<Color>, bg: Option<Color>, decoration: Option<Decoration>) {
    *SUCCESS_STYLE.write().unwrap() = PredefinedStyle { fg, bg, decoration };
}

/// Set the styles for the predefined warning style type
pub fn set_warning_style(fg: Option<Color>, bg: Option<Color>, decoration: Option<Decoration>) {
    *WARNING_STYLE.write().unwrap() = PredefinedStyle { fg, bg, decoration };
}

/// Set the styles for the predefined error style type
pub fn set_error_style(fg: Option<Color>, bg: Option<Color>, decoration: Option<Decoration>) {
    *ERROR_STYLE.write().unwrap() = PredefinedStyle { fg, bg, decoration };
}

/// Set the styles for the predefined info style type
pub fn set_info_style(fg: Option<Color>, bg: Option<Color>, decoration: Option<Decoration>) {
    *INFO_STYLE.write().unwrap() = PredefinedStyle { fg, bg, decoration };
}

/// Set the styles for the predefined debug style type
pub fn set_debug_style(fg: Option<Color>, bg: Option<Color>, decoration: Option<Decoration>) {
    *DEBUG_STYLE.write().unwrap() = PredefinedStyle { fg, bg, decoration };
}

/// Reset the styles for a predefined style type
pub fn reset_predefined_style(style_type: PredefinedStyleType) {
    match style_type {
        PredefinedStyleType::Success => *SUCCESS_STYLE.write().unwrap() = DEFAULT_SUCCESS_STYLE,
        PredefinedStyleType::Warning => *WARNING_STYLE.write().unwrap() = DEFAULT_WARNING_STYLE,
        PredefinedStyleType::Error => *ERROR_STYLE.write().unwrap() = DEFAULT_ERROR_STYLE,
        PredefinedStyleType::Info => *INFO_STYLE.write().unwrap() = DEFAULT_INFO_STYLE,
        PredefinedStyleType::Debug => *DEBUG_STYLE.write().unwrap() = DEFAULT_DEBUG_STYLE,
    }
}

/// Reset the styles for all predefined style types
pub fn reset_all_predefined_styles() {
    *SUCCESS_STYLE.write().unwrap() = DEFAULT_SUCCESS_STYLE;
    *WARNING_STYLE.write().unwrap() = DEFAULT_WARNING_STYLE;
    *ERROR_STYLE.write().unwrap() = DEFAULT_ERROR_STYLE;
    *INFO_STYLE.write().unwrap() = DEFAULT_INFO_STYLE;
    *DEBUG_STYLE.write().unwrap() = DEFAULT_DEBUG_STYLE;
}
