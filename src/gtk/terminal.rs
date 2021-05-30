use gtk::prelude::*;
use relm::*;
use relm_derive::*;

use crate::gtk::app::AppMsg;

#[derive(Msg, Debug)]
pub enum TerminalMsg {
    Quit,
}

pub struct TerminalParams {
    pub stream: StreamHandle<AppMsg>,
}

pub struct TerminalModel {
    stream: StreamHandle<AppMsg>,
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

    fn model(_relm: &Relm<Self>, param: Self::ModelParam) -> Self::Model {
        TerminalModel {
            stream: param.stream,
        }
    }

    fn update(&mut self, event: Self::Msg) {
        println!("{:?}", event);
        match event {
            TerminalMsg::Quit => self
                .model
                .stream
                .emit(AppMsg::TerminalExit(self.root().upcast())),
        }
    }
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
