use gtk::prelude::*;
use relm::*;
use relm_derive::*;

#[derive(Msg)]
pub enum TerminalMsg {}

pub struct TerminalParams {}

pub struct TerminalModel {
    relm: Relm<Terminal>,
}

pub struct Terminal {
    model: TerminalModel,
    // TODO: Make a wrapper around gtk::GLArea and implement Draw, then make this a TerminalDisplay
    display: gtk::GLArea,
}

impl Update for Terminal {
    type Model = TerminalModel;
    type ModelParam = TerminalParams;
    type Msg = TerminalMsg;

    fn model(relm: &Relm<Self>, _param: Self::ModelParam) -> Self::Model {
        TerminalModel { relm: relm.clone() }
    }

    fn update(&mut self, _event: Self::Msg) {}
}

impl Widget for Terminal {
    type Root = gtk::GLArea;

    fn root(&self) -> Self::Root {
        self.display.clone()
    }

    fn view(_relm: &Relm<Self>, model: Self::Model) -> Self {
        Self {
            model,
            display: gtk::GLArea::new(),
        }
    }

    fn init_view(&mut self) {
        self.display.show_all();
    }
}
