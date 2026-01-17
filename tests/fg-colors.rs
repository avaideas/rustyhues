use rustyhues::*;

#[test]
fn foreground_black() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".black());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BLACK_CODE)));
}
#[test]
fn foreground_red() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".red());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_RED_CODE)))
}
#[test]
fn foreground_green() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".green());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_GREEN_CODE)))
}
#[test]
fn foreground_yellow() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".yellow());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_YELLOW_CODE)))
}
#[test]
fn foreground_blue() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".blue());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BLUE_CODE)))
}
#[test]
fn foreground_magenta() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".magenta());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_MAGENTA_CODE)))
}
#[test]
fn foreground_cyan() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".cyan());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_CYAN_CODE)))
}
#[test]
fn foreground_white() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".white());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_WHITE_CODE)))
}
#[test]
fn foreground_bright_black() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bright_black());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BRIGHT_BLACK_CODE)))
}
#[test]
fn foreground_bright_red() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bright_red());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BRIGHT_RED_CODE)))
}
#[test]
fn foreground_bright_green() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bright_green());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BRIGHT_GREEN_CODE)))
}
#[test]
fn foreground_bright_yellow() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bright_yellow());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BRIGHT_YELLOW_CODE)))
}
#[test]
fn foreground_bright_blue() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bright_blue());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BRIGHT_BLUE_CODE)))
}
#[test]
fn foreground_bright_magenta() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bright_magenta());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BRIGHT_MAGENTA_CODE)))
}
#[test]
fn foreground_bright_cyan() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bright_cyan());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BRIGHT_CYAN_CODE)))
}
#[test]
fn foreground_bright_white() {
    env::set_color_choice(env::ColorChoice::Always);
    let painted = format!("{}", "X".bright_white());
    assert!(painted.contains(&format!("\x1b[{}m", COLOR_BRIGHT_WHITE_CODE)))
}
