use alacritty_terminal::term::SizeInfo;

use crate::common::Render;

#[allow(dead_code)]
pub struct Display<R: Render> {
    size_info: SizeInfo,
    renderer: R,
}

impl<R: Render> Display<R> {
    pub fn new(size_info: SizeInfo, renderer: R) -> Self {
        Self {
            size_info,
            renderer,
        }
    }
}
