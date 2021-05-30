use alacritty_terminal::{
    config::Config,
    event::{Event, EventListener},
    tty,
};
use relm::*;
use tracing::*;
use tracing_subscriber::EnvFilter;

mod common;
mod gtk;

use crate::gtk::app::AppParams;

#[derive(Debug, Clone)]
pub struct EventProxy;

impl EventListener for EventProxy {
    #[instrument]
    fn send_event(&self, event: Event) {
        debug!("received event");
    }
}

#[instrument]
fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("starting alacrittab");

    let config: Config<()> = Default::default();
    tty::setup_env(&config);

    gtk::app::App::run(AppParams {
        terminal_config: config,
    })
    .unwrap();
}
