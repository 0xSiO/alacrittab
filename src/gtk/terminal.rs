use std::{io::Write, sync::Arc};

use alacritty_terminal::{
    config::Config,
    event::{Event, EventListener},
    event_loop::{self, EventLoop},
    sync::FairMutex,
    term::SizeInfo,
    tty::{self, Pty},
    Term,
};
use gtk::prelude::*;
use relm::*;
use relm_derive::*;
use tracing::*;

use crate::{
    common::{EventHandler, TerminalDisplay},
    gtk::app::AppMsg,
};

#[derive(Clone)]
pub struct EventProxy {
    stream: Arc<FairMutex<StreamHandle<TerminalMsg>>>,
}

// TODO: Is this ok? Access to the underlying stream is managed by a FairMutex
unsafe impl Send for EventProxy {}

impl EventListener for EventProxy {
    fn send_event(&self, event: Event) {
        self.stream.lock().emit(TerminalMsg::TerminalEvent(event));
    }
}

#[derive(Msg, Debug)]
pub enum TerminalMsg {
    TerminalEvent(Event),
    Render,
    Quit,
}

pub struct TerminalParams {
    pub config: Arc<Config<()>>,
    pub stream: StreamHandle<AppMsg>,
}

pub struct TerminalModel {
    term: Arc<FairMutex<Term<EventProxy>>>,
    event_handler: EventHandler<(), (), EventProxy, Pty>,
    stream: StreamHandle<AppMsg>,
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

    fn model(relm: &Relm<Self>, params: Self::ModelParam) -> Self::Model {
        // TODO: Create size from config
        let size_info = SizeInfo::new(800.0, 600.0, 10.0, 20.0, 5.0, 5.0, false);
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

        let event_handler = EventHandler::new(
            Arc::clone(&term),
            TerminalDisplay::new((), size_info, ()),
            event_loop::Notifier(event_tx),
            io_thread,
        );

        TerminalModel {
            term,
            event_handler,
            stream: params.stream,
            relm: relm.clone(),
        }
    }

    #[instrument(skip(self))]
    fn update(&mut self, event: Self::Msg) {
        use TerminalMsg::*;

        trace!("received event");
        match event {
            TerminalEvent(event) => self.handle_terminal_event(event),
            Render => {
                self.display.make_current();
                unsafe {
                    gl::ClearColor(0., 255., 255., 0.5);
                    gl::Clear(gl::COLOR_BUFFER_BIT);
                }
            }
            Quit => {
                self.model.term.lock().exit();
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

        connect!(
            self.model.relm,
            self.display,
            connect_render(_, event),
            return (TerminalMsg::Render, Inhibit(false))
        );
    }
}

impl Terminal {
    fn handle_terminal_event(&mut self, event: Event) {
        let should_exit = matches!(event, Event::Exit);

        self.model.event_handler.handle(event);

        let term = self.model.term.lock();
        let indexed_cells: Vec<_> = term.renderable_content().display_iter.collect();
        for indexed_cell in indexed_cells {
            print!("{}", indexed_cell.cell.c);
            std::io::stdout().flush().unwrap();
        }

        if should_exit {
            self.model
                .stream
                .emit(AppMsg::TerminalExit(self.root().upcast()));
        }
    }
}
