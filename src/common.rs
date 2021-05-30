use std::{sync::Arc, thread::JoinHandle};

use alacritty_terminal::{
    event::{self, Event, EventListener},
    event_loop::{EventLoop, Msg, Notifier, State},
    sync::FairMutex,
    term::SizeInfo,
    tty, Term,
};
use tracing::*;

// TODO: Methods needed for drawing to surface
pub trait Draw {}

// TODO: Methods needed for rendering
pub trait Render {}

// TODO: Temporary impls while setting things up
impl Draw for () {}
impl Render for () {}

#[allow(dead_code)]
pub struct TerminalDisplay<R, D>
where
    R: Render,
    D: Draw,
{
    renderer: R,
    surface: D,
    size_info: SizeInfo,
}

#[allow(dead_code)]
impl<R, D> TerminalDisplay<R, D>
where
    R: Render,
    D: Draw,
{
    pub fn new(surface: D, size_info: SizeInfo, renderer: R) -> Self {
        Self {
            surface,
            size_info,
            renderer,
        }
    }
}

pub struct EventHandler<R, D, L, P>
where
    R: Render,
    D: Draw,
    L: EventListener,
    P: tty::EventedPty + event::OnResize + Send + 'static,
{
    terminal: Arc<FairMutex<Term<L>>>,
    display: TerminalDisplay<R, D>,
    notifier: Notifier,
    io_thread: Option<JoinHandle<(EventLoop<P, L>, State)>>,
    should_draw: bool,
}

impl<R, D, L, P> EventHandler<R, D, L, P>
where
    R: Render,
    D: Draw,
    L: EventListener,
    P: tty::EventedPty + event::OnResize + Send + 'static,
{
    pub fn new(
        terminal: Arc<FairMutex<Term<L>>>,
        display: TerminalDisplay<R, D>,
        notifier: Notifier,
        io_thread: JoinHandle<(EventLoop<P, L>, State)>,
    ) -> Self {
        Self {
            terminal,
            display,
            notifier,
            io_thread: Some(io_thread),
            should_draw: false,
        }
    }

    #[instrument(skip(self))]
    pub fn handle(&mut self, event: Event) {
        debug!("handling event");

        match event {
            Event::Wakeup => {
                self.should_draw = true;
            }
            Event::Exit => {
                self.notifier.0.send(Msg::Shutdown).unwrap();
                if let Some(io_thread) = self.io_thread.take() {
                    io_thread.join().unwrap();
                }
            }
            _ => todo!(),
        }

        if self.should_draw {
            self.should_draw = false;
            // TODO: Render and draw terminal content to surface
        }
    }
}
