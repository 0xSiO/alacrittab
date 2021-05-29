use gtk::prelude::*;
use relm::*;
use relm_derive::*;

#[derive(Msg)]
pub enum TerminalMsg {}

pub struct TerminalParams {}

pub struct TerminalModel {}

#[widget]
impl Widget for Terminal {
    fn model(_relm: &Relm<Self>, _params: TerminalParams) -> TerminalModel {
        TerminalModel {}
    }

    fn update(&mut self, event: TerminalMsg) {
        match event {}
    }

    view! {
        #[name = "gl_area"]
        gtk::GLArea {}
    }

    fn init_view(&mut self) {
        self.widgets.gl_area.show_all();
    }
}
