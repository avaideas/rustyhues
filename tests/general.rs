use rustyhues::*;

#[test]
fn add_reset() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".blue());
    assert!(painted.ends_with("\x1b[0m") || painted == "X");
}

#[test]
fn reset_only_appears_once() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".blue().bg_white().bold());
    assert_eq!(painted.matches("\x1b[0m").count(), 1)
}

#[test]
fn chain_styles() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".blue().bg_white().bold());
    assert!(painted.contains(&format!(
        "\x1b[{};{};{}m",
        COLOR_BLUE_CODE, COLOR_BG_WHITE_CODE, DECORATION_BOLD_CODE
    )))
}

#[test]
fn paint_with_no_styles() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".paint());
    assert_eq!(painted, "X")
}

#[test]
fn separators_present_when_multiple_styles() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".blue().bold());
    assert!(painted.starts_with("\x1b[34;1m"))
}

#[test]
fn paint_err_smoke() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".paint_err().red());
    assert!(painted.contains("\x1b[31m") || painted == "X")
}

#[test]
fn color_choice_never() {
    env::set_color_choice(env::ColorChoice::Never);
    let painted = format!("{}", Paint::new_stdout("X").red().bg_bright_cyan().bold());
    assert_eq!(painted, "X");
}

#[test]
fn should_reset_false() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".blue().should_reset(false));
    assert!(!painted.contains("\x1b[0m"));
}

#[test]
fn single_is_true() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".is(Some(1 + 1 == 2)).blue());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BLUE_CODE)));
}

#[test]
fn single_is_false() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".is(Some(1 + 1 == 1)).blue());
    assert!(!painted.contains(&format!("\x1b[{}m", COLOR_BLUE_CODE)));
}

#[test]
fn single_not_true() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".not(Some(1 + 1 == 1)).blue());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BLUE_CODE)));
}

#[test]
fn single_not_false() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".not(Some(1 + 1 == 2)).blue());
    assert!(!painted.contains(&format!("\x1b[{}m", COLOR_BLUE_CODE)));
}

#[test]
fn simple_is_true() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".blue().is(Some(10 > 5)).bg_white());
    assert!(painted.contains(&format!(
        "\x1b[{};{}m",
        COLOR_BLUE_CODE, COLOR_BG_WHITE_CODE
    )));
}

#[test]
fn simple_is_false() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".blue().is(Some(10 < 5)).bg_white());
    assert!(!painted.contains(&format!(
        "\x1b[{};{}m",
        COLOR_BLUE_CODE, COLOR_BG_WHITE_CODE
    )));
}

#[test]
fn simple_not_true() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".blue().not(Some(10 < 5)).bg_white());
    assert!(painted.contains(&format!(
        "\x1b[{};{}m",
        COLOR_BLUE_CODE, COLOR_BG_WHITE_CODE
    )));
}

#[test]
fn simple_not_false() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".blue().not(Some(10 > 5)).bg_white());
    assert!(!painted.contains(&format!(
        "\x1b[{};{}m",
        COLOR_BLUE_CODE, COLOR_BG_WHITE_CODE
    )));
}

#[test]
fn chain_is_not_optional() {
    let test_vec: Vec<String> = Vec::new();
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!(
        "{}",
        "X".is(Some(test_vec.is_empty())).green().not(None).red()
    );
    assert!(
        painted.contains(&format!("\x1b[{}m", COLOR_GREEN_CODE))
            && !painted.contains(&format!("\x1b[{}m", COLOR_RED_CODE))
    )
}

#[test]
fn chain_is_not_fixed() {
    let test_vec: Vec<String> = Vec::new();
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".e_is(test_vec.is_empty()).green().n_not().red());
    assert!(
        painted.contains(&format!("\x1b[{}m", COLOR_GREEN_CODE))
            && !painted.contains(&format!("\x1b[{}m", COLOR_RED_CODE))
    )
}

#[test]
fn success() {
    env::set_color_choice(env::ColorChoice::Always);
    env::reset_predefined_style(env::PredefinedStyleType::Success);
    let painted = format!("{}", "X".success());
    assert!(painted.contains(&format!(
        "\x1b[{};{}m",
        COLOR_GREEN_CODE, DECORATION_BOLD_CODE
    )))
}

#[test]
fn warning() {
    env::set_color_choice(env::ColorChoice::Always);
    env::reset_predefined_style(env::PredefinedStyleType::Warning);
    let painted = format!("{}", "X".warning());
    assert!(painted.contains(&format!(
        "\x1b[{};{}m",
        COLOR_YELLOW_CODE, DECORATION_BOLD_CODE
    )))
}

#[test]
fn error() {
    env::set_color_choice(env::ColorChoice::Always);
    env::reset_predefined_style(env::PredefinedStyleType::Error);
    let painted = format!("{}", "X".error());
    assert!(painted.contains(&format!(
        "\x1b[{};{}m",
        COLOR_RED_CODE, DECORATION_BOLD_CODE
    )))
}

#[test]
fn info() {
    env::set_color_choice(env::ColorChoice::Always);
    env::reset_predefined_style(env::PredefinedStyleType::Info);
    let painted = format!("{}", "X".info());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BLUE_CODE)))
}

#[test]
fn debug() {
    env::set_color_choice(env::ColorChoice::Always);
    env::reset_predefined_style(env::PredefinedStyleType::Debug);
    let painted = format!("{}", "X".debug());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_MAGENTA_CODE)))
}

#[test]
fn success_custom() {
    env::set_color_choice(env::ColorChoice::Always);
    env::reset_predefined_style(env::PredefinedStyleType::Success);
    env::set_predefined_style(
        env::PredefinedStyleType::Success,
        Some(Color::Cyan),
        Some(Color::White),
        Some(Decoration::Underline),
    );
    let painted = format!("{}", "X".success());
    assert!(
        !painted.contains(&format!(
            "\x1b[{};{}m",
            COLOR_GREEN_CODE, DECORATION_BOLD_CODE
        )) && painted.contains(&format!(
            "\x1b[{};{};{}m",
            COLOR_CYAN_CODE, COLOR_BG_WHITE_CODE, DECORATION_UNDERLINE_CODE
        ))
    )
}

#[test]
fn warning_custom() {
    env::set_color_choice(env::ColorChoice::Always);
    env::reset_predefined_style(env::PredefinedStyleType::Warning);
    env::set_predefined_style(
        env::PredefinedStyleType::Warning,
        Some(Color::Cyan),
        Some(Color::White),
        Some(Decoration::Underline),
    );
    let painted = format!("{}", "X".warning());
    assert!(
        !painted.contains(&format!(
            "\x1b[{};{}m",
            COLOR_YELLOW_CODE, DECORATION_BOLD_CODE
        )) && painted.contains(&format!(
            "\x1b[{};{};{}m",
            COLOR_CYAN_CODE, COLOR_BG_WHITE_CODE, DECORATION_UNDERLINE_CODE
        ))
    )
}

#[test]
fn error_custom() {
    env::set_color_choice(env::ColorChoice::Always);
    env::reset_predefined_style(env::PredefinedStyleType::Error);
    env::set_predefined_style(
        env::PredefinedStyleType::Error,
        Some(Color::Cyan),
        Some(Color::White),
        Some(Decoration::Underline),
    );
    let painted = format!("{}", "X".error());
    assert!(
        !painted.contains(&format!(
            "\x1b[{};{}m",
            COLOR_RED_CODE, DECORATION_BOLD_CODE
        )) && painted.contains(&format!(
            "\x1b[{};{};{}m",
            COLOR_CYAN_CODE, COLOR_BG_WHITE_CODE, DECORATION_UNDERLINE_CODE
        ))
    )
}

#[test]
fn info_custom() {
    env::set_color_choice(env::ColorChoice::Always);
    env::reset_predefined_style(env::PredefinedStyleType::Info);
    env::set_predefined_style(
        env::PredefinedStyleType::Info,
        Some(Color::Cyan),
        Some(Color::White),
        Some(Decoration::Underline),
    );
    let painted = format!("{}", "X".info());
    assert!(
        !painted.contains(&format!("\x1b[{}m", COLOR_BLUE_CODE))
            && painted.contains(&format!(
                "\x1b[{};{};{}m",
                COLOR_CYAN_CODE, COLOR_BG_WHITE_CODE, DECORATION_UNDERLINE_CODE
            ))
    )
}

#[test]
fn debug_custom() {
    env::set_color_choice(env::ColorChoice::Always);
    env::reset_predefined_style(env::PredefinedStyleType::Debug);
    env::set_predefined_style(
        env::PredefinedStyleType::Debug,
        Some(Color::Cyan),
        Some(Color::White),
        Some(Decoration::Underline),
    );
    let painted = format!("{}", "X".debug());
    assert!(
        !painted.contains(&format!("\x1b[{}m", COLOR_MAGENTA_CODE))
            && painted.contains(&format!(
                "\x1b[{};{};{}m",
                COLOR_CYAN_CODE, COLOR_BG_WHITE_CODE, DECORATION_UNDERLINE_CODE
            ))
    )
}
