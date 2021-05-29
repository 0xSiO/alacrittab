use alacritty_terminal::term::SizeInfo;

// TODO: Methods needed for drawing to surface
pub trait Draw {}

// TODO: Methods needed for OpenGL rendering
pub trait Render {}

#[allow(dead_code)]
pub struct TerminalDisplay<R: Render, D: Draw> {
    renderer: R,
    surface: D,
    size_info: SizeInfo,
}

#[allow(dead_code)]
impl<R: Render, D: Draw> TerminalDisplay<R, D> {
    pub fn new(surface: D, size_info: SizeInfo, renderer: R) -> Self {
        Self {
            surface,
            size_info,
            renderer,
        }
    }
}
