use alacritty_terminal::term::SizeInfo;

use crate::common::Render;

#[allow(dead_code)]
pub struct Display<R: Render> {
    renderer: R,
    size_info: SizeInfo,
}

impl<R: Render> Display<R> {
    pub fn new(size_info: SizeInfo, renderer: R) -> Self {
        Self {
            size_info,
            renderer,
        }
    }
}
