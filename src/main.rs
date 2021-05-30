use alacritty_terminal::{
    config::Config,
    event::{Event, EventListener},
    tty,
};
use relm::*;

mod common;
mod gtk;

use crate::gtk::app::AppParams;

#[derive(Clone)]
pub struct EventProxy;

impl EventListener for EventProxy {
    fn send_event(&self, event: Event) {
        println!("Event: {:?}", event);
    }
}

fn main() {
    let config: Config<()> = Default::default();
    tty::setup_env(&config);

    gtk::app::App::run(AppParams {
        terminal_config: config,
    })
    .unwrap();
}
