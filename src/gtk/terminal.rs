use alacritty_terminal::{
    event_loop::{EventLoop, Msg, State},
    sync::FairMutex,
    tty::Pty,
    Term,
};
use gtk::prelude::*;
use relm::*;
use relm_derive::*;
use std::{sync::Arc, thread::JoinHandle};

use crate::{gtk::app::AppMsg, EventProxy};

#[derive(Msg, Debug)]
pub enum TerminalMsg {
    Quit,
}

pub struct TerminalParams {
    pub stream: StreamHandle<AppMsg>,
    pub term: Arc<FairMutex<Term<EventProxy>>>,
    pub loop_tx: mio_extras::channel::Sender<Msg>,
    pub io_thread: JoinHandle<(EventLoop<Pty, EventProxy>, State)>,
}

pub struct TerminalModel {
    stream: StreamHandle<AppMsg>,
    term: Arc<FairMutex<Term<EventProxy>>>,
    loop_tx: mio_extras::channel::Sender<Msg>,
    io_thread: Option<JoinHandle<(EventLoop<Pty, EventProxy>, State)>>,
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

    fn model(_relm: &Relm<Self>, params: Self::ModelParam) -> Self::Model {
        TerminalModel {
            stream: params.stream,
            term: params.term,
            loop_tx: params.loop_tx,
            io_thread: Some(params.io_thread),
        }
    }

    fn update(&mut self, event: Self::Msg) {
        println!("{:?}", event);
        match event {
            TerminalMsg::Quit => {
                self.model.loop_tx.send(Msg::Shutdown).unwrap();
                if let Some(io_thread) = self.model.io_thread.take() {
                    io_thread.join().unwrap();
                }

                self.model
                    .stream
                    .emit(AppMsg::TerminalExit(self.root().upcast()));
            }
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
