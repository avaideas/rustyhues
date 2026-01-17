use rustyhues::*;

#[test]
fn decoration_bold() {
    let painted = format!("{}", "X".bold());
    assert!(painted.contains(&format!("\x1b[{}m", DECORATION_BOLD_CODE)))
}
#[test]
fn decoration_dim() {
    let painted = format!("{}", "X".dim());
    assert!(painted.contains(&format!("\x1b[{}m", DECORATION_DIM_CODE)))
}
#[test]
fn decoration_italic() {
    let painted = format!("{}", "X".italic());
    assert!(painted.contains(&format!("\x1b[{}m", DECORATION_ITALIC_CODE)))
}
#[test]
fn decoration_underline() {
    let painted = format!("{}", "X".underline());
    assert!(painted.contains(&format!("\x1b[{}m", DECORATION_UNDERLINE_CODE)))
}
#[test]
fn decoration_invert() {
    let painted = format!("{}", "X".invert());
    assert!(painted.contains(&format!("\x1b[{}m", DECORATION_INVERT_CODE)))
}
