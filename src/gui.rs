use gtk::prelude::*;

use crate::notebook::Notebook;

mod state;

use state::{AppState, GuiState};

pub fn initialize(application: &gtk::Application) {
    let (_app, gui) = create_state(application);

    let GuiState {
        window,
        header_bar,
        notebook,
        ..
    } = &gui;

    // Notebook setup
    for i in 1..4 {
        let title = format!("Terminal {}", i);
        let label = gtk::Label::new(Some(&*title));
        notebook.add_tab(&title, label);
    }

    // HeaderBar setup
    header_bar.set_title(Some("Alacrittab"));
    header_bar.set_show_close_button(true);

    // Window setup
    window.set_title("Notebook");
    window.set_titlebar(Some(header_bar));
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(640, 480);
    window.add(&notebook.inner);
    window.show_all();
}

fn create_state(application: &gtk::Application) -> (AppState, GuiState) {
    let window = gtk::ApplicationWindow::new(application);
    let header_bar = gtk::HeaderBar::new();
    let notebook = Notebook::new();

    let app = AppState {};
    let gui = GuiState {
        application: application.clone(),
        window,
        header_bar,
        notebook,
    };

    (app, gui)
}
