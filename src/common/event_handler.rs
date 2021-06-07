use std::{sync::Arc, thread::JoinHandle};

use alacritty_terminal::{
    event::{Event, EventListener},
    event_loop::{EventLoop, Msg, Notifier, State},
    sync::FairMutex,
    tty::Pty,
    Term,
};
use tracing::*;

use crate::common::{Display, Render};

#[allow(dead_code)]
pub struct EventHandler<R, L>
where
    R: Render,
    L: EventListener,
{
    terminal: Arc<FairMutex<Term<L>>>,
    display: Display<R>,
    notifier: Notifier,
    io_thread: Option<JoinHandle<(EventLoop<Pty, L>, State)>>,
    should_draw: bool,
}

impl<R, L> EventHandler<R, L>
where
    R: Render,
    L: EventListener,
{
    pub fn new(
        terminal: Arc<FairMutex<Term<L>>>,
        display: Display<R>,
        notifier: Notifier,
        io_thread: JoinHandle<(EventLoop<Pty, L>, State)>,
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
            _ => {}
        }

        if self.should_draw {
            self.should_draw = false;
            // TODO: Render and draw terminal content to surface
        }
    }
}
