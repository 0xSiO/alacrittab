use std::{sync::Arc, thread::JoinHandle};

use alacritty_terminal::{
    config::Config,
    event::{Event, EventListener},
    event_loop::{EventLoop, Msg, State},
    sync::FairMutex,
    term::SizeInfo,
    tty::{self, Pty},
    Term,
};
use gtk::prelude::*;
use relm::*;
use relm_derive::*;
use tracing::*;

use crate::gtk::app::AppMsg;

#[derive(Clone)]
pub struct EventProxy {
    stream: Arc<FairMutex<StreamHandle<TerminalMsg>>>,
}

// TODO: Is this ok? Access to the underlying stream is managed by a FairMutex
unsafe impl Send for EventProxy {}

impl EventListener for EventProxy {
    fn send_event(&self, event: Event) {
        self.stream.lock().emit(TerminalMsg::Event(event));
    }
}

#[derive(Msg, Debug)]
pub enum TerminalMsg {
    Event(Event),
    Quit,
}

pub struct TerminalParams {
    pub stream: StreamHandle<AppMsg>,
    pub config: Arc<Config<()>>,
}

pub struct TerminalModel {
    stream: StreamHandle<AppMsg>,
    term: Arc<FairMutex<Term<EventProxy>>>,
    event_tx: mio_extras::channel::Sender<Msg>,
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

    fn model(relm: &Relm<Self>, params: Self::ModelParam) -> Self::Model {
        // TODO: Create size from config
        let size_info = SizeInfo::new(1024.0, 768.0, 10.0, 20.0, 5.0, 5.0, false);
        let event_proxy = EventProxy {
            stream: Arc::new(FairMutex::new(relm.stream().clone())),
        };

        let term = Arc::new(FairMutex::new(Term::new(
            &params.config,
            size_info,
            event_proxy.clone(),
        )));

        let pty = tty::new(&params.config, &size_info, None);
        let event_loop = EventLoop::new(Arc::clone(&term), event_proxy, pty, false, false);
        let event_tx = event_loop.channel();
        let io_thread = event_loop.spawn();

        TerminalModel {
            stream: params.stream,
            term,
            event_tx,
            io_thread: Some(io_thread),
        }
    }

    #[instrument(skip(self))]
    fn update(&mut self, event: Self::Msg) {
        debug!("received event");
        match event {
            TerminalMsg::Event(event) => match event {
                Event::Wakeup => {
                    // TODO: Queue up a draw
                }
                // TODO: Handle other alacritty_terminal events
                _ => {}
            },
            TerminalMsg::Quit => {
                self.model.event_tx.send(Msg::Shutdown).unwrap();
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
