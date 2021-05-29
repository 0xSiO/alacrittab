use std::sync::Arc;

use alacritty_terminal::{
    config::Config,
    event::{Event, EventListener},
    event_loop::{EventLoop, Msg},
    sync::FairMutex,
    term::SizeInfo,
    tty, Term,
};
use relm::*;

mod app;
mod tab;
mod terminal_display;

#[derive(Clone)]
struct EventProxy;

impl EventListener for EventProxy {
    fn send_event(&self, event: Event) {
        println!("Event: {:?}", event);
    }
}

fn main() {
    let config: Config<()> = Default::default();
    tty::setup_env(&config);

    let size_info = SizeInfo::new(1024.0, 768.0, 10.0, 20.0, 5.0, 5.0, false);
    let event_proxy = EventProxy;

    let terminal = Arc::new(FairMutex::new(Term::new(
        &config,
        size_info,
        event_proxy.clone(),
    )));

    let pty = tty::new(&config, &size_info, None);

    let pty_event_loop = EventLoop::new(
        Arc::clone(&terminal),
        event_proxy.clone(),
        pty,
        false,
        false,
    );

    let loop_tx = pty_event_loop.channel();

    let io_thread = pty_event_loop.spawn();

    app::App::run(()).unwrap();

    loop_tx.send(Msg::Shutdown).unwrap();
    io_thread.join().unwrap();
}
