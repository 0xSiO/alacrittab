use std::ptr;

use alacritty_terminal::{config::Config, tty};
use relm4::RelmApp;
use shared_library::dynamic_library::DynamicLibrary;
use tracing::*;
use tracing_subscriber::EnvFilter;

mod common;
mod gtk;

use crate::gtk::app::AppModel;

// TODO: Note required packages: libepoxy-devel, gtk4-devel, libadwaita
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

    epoxy::load_with(|s| unsafe {
        match DynamicLibrary::open(None).unwrap().symbol(s) {
            Ok(v) => v,
            Err(_) => ptr::null(),
        }
    });
    gl::load_with(epoxy::get_proc_addr);

    RelmApp::new(AppModel::default()).run();

    info!("quitting alacrittab");
}
