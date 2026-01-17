# RustyHues

Lightweight, zero-macro ANSI color styling for Rust CLIs, with smart terminal detection, RGB/hex support, conditional styling, and configurable semantic styles like `success()`, `warning()`, and `error()`.

```rust
use rustyhues::Stylize;

fn main() {
    println!("{}", "Hello, RustyHues!".blue().bold());
}
```

## Features

- Ergonomic API - `"text".blue().bold()` via a blanket Stylize trait.
- RGB & Hex colors - `rgb(255, 0, 128)` and `hex("ff0080")` or `hex("f08")`.
- Truecolor with graceful fallback - uses 24-bit color when supported, falls back to the nearest ANSI color otherwise.
- Smart color detection - respects `NO_COLOR`, `CLICOLOR`, `CLICOLOR_FORCE`, `TERM=dumb`, and TTY detection.
- Semantic styles - `success()`, `warning()`, `error()`, `info()`, and `debug()` with configurable palettes.
- Conditional styling - enable or disable styles based on runtime expressions with `.is()` / `.not()`.
- Cross-platform - Unix & Windows (enables virtual terminal processing on modern Windows consoles).
- Opt-out / opt-in - globally force on/off, or let RustyHues auto-detect.

## Installation

Add RustyHues to your project:

```shell
cargo add rustyhues
```

Or manually in Cargo.toml:

```toml
[dependencies]
rustyhues = "1.0"
```

## Quick Start

```rust
use rustyhues::Stylize;

fn main() {
    println!("{}", "Success".green().bold());
    println!("{}", "Warning".yellow());
    eprintln!("{}", "Error on stderr".red().paint_err());
}
```

Under the hood:

- Stylize is implemented for all types, so you can call methods directly on `&str`, `String`, etc.
- Each call returns a `Paint<T>` wrapper that implements `Display`, so it fits into `println!`, `format!`, logs, etc.

## Basic Styling

**Named ANSI colors:**

```rust
use rustyhues::Stylize;

// Standard ANSI colors
println!("{}", "Black".black());
println!("{}", "Red".red());
println!("{}", "Green".green());
println!("{}", "Yellow".yellow());
println!("{}", "Blue".blue());
println!("{}", "Magenta".magenta());
println!("{}", "Cyan".cyan());
println!("{}", "White".white());

// Bright variants
println!("{}", "Bright Red".bright_red());
println!("{}", "Bright Blue".bright_blue());
```

**Background colors:**

```rust
use rustyhues::Stylize;

println!("{}", "On red".bg_red());
println!("{}", "Bright on white".blue().bg_bright_white());
```

**Text decorations:**

```rust
use rustyhues::Stylize;

println!("{}", "Bold".bold());
println!("{}", "Dim".dim());
println!("{}", "Italic".italic());
println!("{}", "Underline".underline());
println!("{}", "Inverted".invert());
```

You can chain as many styles as you like:

```rust
use rustyhues::Stylize;

println!("{}", "Fancy".cyan().bg_bright_black().bold().underline());
```

## RGB & Hex Colors

RustyHues supports full 24-bit color (when the terminal does) and gracefully falls back to the nearest ANSI color if not.

### RGB

```rust
use rustyhues::Stylize;

println!("{}", "Custom RGB".rgb(255, 128, 64));
println!("{}", "RGB background".bg_rgb(10, 20, 30));
```

### Hex (3 or 6 chars, with or without #)

```rust
use rustyhues::Stylize;

println!("{}", "Hex color".hex("ff8844"));
println!("{}", "Hex background".bg_hex("#333333"));
println!("{}", "Short hex".hex("F84")); 
println!("{}", "Short bg hex".bg_hex("#333"));
```

Invalid hex values simply don't apply any extra style (they don't panic):

```rust
use rustyhues::Stylize;

let painted = format!("{}", "X".blue().hex("ZZZZZZ"));
assert!(painted.starts_with("\x1b[34m")); // blue is still applied
```

## Semantic Styles (success, warning, error, info, debug)

For log-style output, RustyHues comes with predefined styles:

```rust
use rustyhues::Stylize;

println!("{}", "All good".success());
println!("{}", "Heads up".warning());
println!("{}", "Something broke".error());
println!("{}", "FYI".info());
println!("{}", "Debug details".debug());
```

By default:

- success = green + bold
- warning = yellow + bold
- error = red + bold
- info = blue
- debug = magenta

You can customize these globally via `rustyhues::env`:

```rust
use rustyhues::Stylize;
use rustyhues::{env, Color, Decoration};

fn main() {
    // Change success style to cyan text on white background, underlined
    env::set_success_style(
        Some(Color::Cyan),
        Some(Color::White),
        Some(Decoration::Underline),
    );

    println!("{}", "Success?".success());

    // Reset all predefined styles back to defaults
    env::reset_all_predefined_styles();
}
```

Or target a specific style:

```rust
use rustyhues::{env, Color, Decoration};

// Change warning style to bright yellow text, bold
env::set_warning_style(
    Some(Color::BrightYellow),
    None,
    Some(Decoration::Bold),
);
```

You can also use the generic setter:

```rust
use rustyhues::{env, Color, Decoration};

env::set_predefined_style(
    env::PredefinedStyleType::Error,
    Some(Color::BrightRed),
    None,
    Some(Decoration::Bold),
);
```

## Conditional Styling (`is` / `not`)

A unique RustyHues feature is conditional styling based on runtime expressions. This lets you write logic like:

> "Only apply this style if this condition is true."

Simple case:

```rust
use rustyhues::Stylize;

fn main() {
    let debug_mode = true;

    println!(
        "{}",
        "Debug details".is(Some(debug_mode)).magenta()
    );
}
```

- If `debug_mode == true`, the text is magenta.
- If `debug_mode == false`, no styles are applied.

### `is`, `not`, `e_is`, `e_not`, `n_is`, `n_not`

A few helper methods are supplied so that `Some()` and `None` don't have to be typed. These are:

- `e_is` - Expression is
- `e_not` - Expression is not
- `n_is` - None is
- `n_not` - None is not

You can get quite expressive:

```rust
use rustyhues::Stylize;

let is_ok = true;

// Equivalent ways to say “if is_ok then style, otherwise don't”:
println!("{}", "OK".is(Some(is_ok)).green());
println!("{}", "OK".e_is(is_ok).green());

// Invert the condition:
println!("{}", "Not OK".not(Some(is_ok)).red());
println!("{}", "Not OK".e_not(is_ok).red());

// Toggle style with no explicit expression:
println!("{}", "Maybe styled".n_is().blue());  // toggles on/off
println!("{}", "Maybe not".n_not().red());     // toggles on/off

// Use as if / else
println!("{}", "Maybe Ok".e_is(is_ok).green().n_not().red());
```

You can chain comparisons in complex expressions. RustyHues internally tracks whether styles should be applied as it walks the items you've built up.

## stdout vs stderr (paint / paint_err)

By default, styling assumes `stdout`. If you want a Paint explicitly targeting `stderr`, use `paint_err()`:

```rust
use rustyhues::Stylize;

eprintln!("{}", "This goes to stderr".paint_err().red().bold());
```

Both stdout and stderr obey the same environment/TTY detection rules.

## Controlling Reset Behavior (should_reset / no_reset)

By default, RustyHues writes a reset `\x1b[0m` at the end of your string so that styles don't continue to subsequent output.

Sometimes you want them to continue, for example when you're building custom prompts:

```rust
use rustyhues::Stylize;

print!("{}", ">>> ".green().no_reset());
// terminal remains green after this
println!("still green");
```

You can also be explicit:

```rust
use rustyhues::Stylize;

println!("{}", "No reset".blue().should_reset(false));
println!("{}", "No reset".blue().should_reset(true));
```

## Global Configuration (env module)

The env module controls global behavior.

### Color choice

```rust
use rustyhues::env::{self, ColorChoice};

env::set_color_choice(ColorChoice::Always); // always style
env::set_color_choice(ColorChoice::Never);  // never style
env::set_color_choice(ColorChoice::Auto);   // (default) auto-detect
```

- Auto (default) uses environment variables + TTY detection.
- Always ignores detection and forces styles on.
- Never disables styles completely.

You can read the current value with:

```rust
let current = env::color_choice();
```

### Truecolor (24-bit) support

```rust
use rustyhues::env;

// Let RustyHues decide (based on COLORTERM, etc.)
env::set_true_color_allowed(None);

// Force allow truecolor
env::set_true_color_allowed(Some(true));

// Force disable truecolor (always fall back to nearest ANSI color)
env::set_true_color_allowed(Some(false));
```

Internally, when truecolor is disabled, RustyHues finds the nearest ANSI color using Euclidean distance in RGB space.

### Environment Variables & Detection

When `ColorChoice::Auto` is active (the default), RustyHues follows common conventions:

- `NO_COLOR` set - disables colors.
- `TERM=dumb` - disables colors.
- `CLICOLOR=0` - disables colors.
- `CLICOLOR_FORCE=1` - force enables colors.
- Otherwise - enables colors only if the target stream (stdout/stderr) is a TTY.

On Unix, TTY detection uses `libc::isatty`.
On Windows, RustyHues uses the Windows API to:

- Detect console handles.
- Enable VT processing (`ENABLE_VIRTUAL_TERMINAL_PROCESSING`) so ANSI escape codes are supported in modern terminals.

## Testing Tips

RustyHues uses some global mutable state in env (color choice, truecolor, predefined styles). This is great for configuring an app, but just be aware in tests:

- Tests that change global settings can affect each other if run in parallel.

Recommended patterns:

```rust
use rustyhues::env::{self, ColorChoice};

fn setup_colors_for_tests() {
    env::set_color_choice(ColorChoice::Always);
    env::set_true_color_allowed(Some(false)); // stable ANSI output
    env::reset_all_predefined_styles();
}
```

You can call this at the start of relevant tests. If you see flaky tests due to ordering, consider:

- Running with a single thread: `cargo test -- --test-threads=1`, or
- Grouping RustyHues-using tests in a dedicated module that manages setup/teardown.

## API Overview

Publicly exported types and modules you'll commonly use:

### Traits

#### Stylize

- `paint()`, `paint_err()`
- `rgb`, `bg_rgb`, `hex`, `bg_hex`
- Named colors: `red`, `bg_red`, `bright_red`, `bg_bright_red`, etc...
- Decorations: `bold`, `dim`, `italic`, `underline`, `invert`
- Conditional: `is()`, `not()`, `e_is()`, `e_not()`, `n_is()`, `n_not()`
- Semantic: `success()`, `warning()`, `error()`, `info()`, `debug()`
- Reset Control: `should_reset()`, `no_reset()`

### Core Types

- `Paint<T>` - styling wrapper that implements `Display`.
- `Color` - ANSI color enum.
- `Decoration` - ANSI text decoration enum.
- `RGB` - RGB struct with helpers (including `closest_color()`).
- `Comparison` - Is / Not for conditional styling.

### Configuration

- `env::ColorChoice` - Auto, Always, Never.
- `env::set_color_choice`, `env::color_choice`
- `env::set_true_color_allowed`, `env::true_color_allowed`

### Predefined Styles

- `env::PredefinedStyleType`
- `env::predefined_style`
- `env::set_predefined_style`
- `env::set_success_style`, `env::set_warning_style`, `env::set_error_style`, `env::set_info_style`, `env::set_debug_style`
- `env::reset_predefined_style`
- `env::reset_all_predefined_styles`

## License

RustyHues is dual-licensed under:

- [MIT License](/LICENSE-MIT)
- [Apache License, Version 2.0](/LICENSE-APACHE)

You may choose either license.

Enjoy painting your terminal with RustyHues
