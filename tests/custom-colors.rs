use rustyhues::*;

/* RGB Tests */
#[test]
fn foreground_rgb() {
    env::set_color_choice(env::ColorChoice::Always);
    env::set_true_color_allowed(Some(true));
    let painted = format!("{}", "X".rgb(255, 254, 253));
    assert!(painted.contains(&format!("\x1b[{}255;254;253m", COLOR_RGB)))
}

#[test]
fn background_rgb() {
    env::set_color_choice(env::ColorChoice::Always);
    env::set_true_color_allowed(Some(true));
    let painted = format!("{}", "X".bg_rgb(255, 254, 253));
    assert!(painted.contains(&format!("\x1b[{}255;254;253m", COLOR_BG_RGB)))
}

/* HEX Tests */
#[test]
fn foreground_full_hex() {
    env::set_color_choice(env::ColorChoice::Always);
    env::set_true_color_allowed(Some(true));
    let painted = format!("{}", "X".hex("fffefd"));
    assert!(painted.contains(&format!("\x1b[{}255;254;253m", COLOR_RGB)))
}

#[test]
fn background_full_hex() {
    env::set_color_choice(env::ColorChoice::Always);
    env::set_true_color_allowed(Some(true));
    let painted = format!("{}", "X".bg_hex("fffefd"));
    assert!(painted.contains(&format!("\x1b[{}255;254;253m", COLOR_BG_RGB)))
}

#[test]
fn foreground_short_hex() {
    env::set_color_choice(env::ColorChoice::Always);
    env::set_true_color_allowed(Some(true));
    let painted = format!("{}", "X".hex("F1D"));
    assert!(painted.contains(&format!("\x1b[{}255;17;221m", COLOR_RGB)))
}

#[test]
fn background_short_hex() {
    env::set_color_choice(env::ColorChoice::Always);
    env::set_true_color_allowed(Some(true));
    let painted = format!("{}", "X".bg_hex("F1D"));
    assert!(painted.contains(&format!("\x1b[{}255;17;221m", COLOR_BG_RGB)))
}

#[test]
fn invalid_length_hex() {
    env::set_color_choice(env::ColorChoice::Always);
    env::set_true_color_allowed(Some(true));
    let painted = format!("{}", "X".hex("1111"));
    assert!(painted.eq("X"))
}

#[test]
fn invalid_format_hex() {
    env::set_color_choice(env::ColorChoice::Always);
    env::set_true_color_allowed(Some(true));
    let painted = format!("{}", "X".hex("ZZZZZZ"));
    assert!(painted.eq("X"))
}

#[test]
fn rgb_falls_back_if_truecolor_disabled() {
    env::set_color_choice(env::ColorChoice::Always);
    env::set_true_color_allowed(Some(false));
    let painted = format!("{}", "X".rgb(255, 255, 255));
    assert!(painted.contains("\x1b[97m"));
}

#[test]
fn invalid_hex_does_not_break_styles() {
    env::set_color_choice(env::ColorChoice::Always);
    env::set_true_color_allowed(Some(true));
    let painted = format!("{}", "X".blue().hex("ZZZ"));
    assert!(painted.starts_with("\x1b[34m"));
    assert!(!painted.contains("38;2;"));
}

#[test]
fn short_hex_expands_correctly() {
    env::set_color_choice(env::ColorChoice::Always);
    env::set_true_color_allowed(Some(true));
    let painted = format!("{}", "X".hex("F1D"));
    assert!(painted.contains("\x1b[38;2;255;17;221m"));
}
