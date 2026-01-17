use rustyhues::*;

#[test]
fn background_black() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted: String = format!("{}", "X".bg_black());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_BLACK_CODE)));
}
#[test]
fn background_red() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_red());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_RED_CODE)))
}
#[test]
fn background_green() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_green());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_GREEN_CODE)))
}
#[test]
fn background_yellow() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_yellow());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_YELLOW_CODE)))
}
#[test]
fn background_blue() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_blue());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_BLUE_CODE)))
}
#[test]
fn background_magenta() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_magenta());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_MAGENTA_CODE)))
}
#[test]
fn background_cyan() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_cyan());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_CYAN_CODE)))
}
#[test]
fn background_white() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_white());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_WHITE_CODE)))
}
#[test]
fn background_bright_black() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_bright_black());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_BRIGHT_BLACK_CODE)))
}
#[test]
fn background_bright_red() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_bright_red());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_BRIGHT_RED_CODE)))
}
#[test]
fn background_bright_green() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_bright_green());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_BRIGHT_GREEN_CODE)))
}
#[test]
fn background_bright_yellow() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_bright_yellow());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_BRIGHT_YELLOW_CODE)))
}
#[test]
fn background_bright_blue() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_bright_blue());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_BRIGHT_BLUE_CODE)))
}
#[test]
fn background_bright_magenta() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_bright_magenta());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_BRIGHT_MAGENTA_CODE)))
}
#[test]
fn background_bright_cyan() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_bright_cyan());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_BRIGHT_CYAN_CODE)))
}
#[test]
fn background_bright_white() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bg_bright_white());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BG_BRIGHT_WHITE_CODE)))
}
