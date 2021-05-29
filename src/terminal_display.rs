use alacritty_terminal::term::SizeInfo;

pub trait Draw {}

pub struct TerminalDisplay<D: Draw> {
    surface: D,
    size_info: SizeInfo,
    renderer: (), // TODO
}

impl<D: Draw> TerminalDisplay<D> {
    pub fn new(surface: D, size_info: SizeInfo, renderer: ()) -> Self {
        Self {
            surface,
            size_info,
            renderer,
        }
    }
}
