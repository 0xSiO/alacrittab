use gio::prelude::*;

mod gui;
mod notebook;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.notebook"),
        Default::default(),
    )
    .expect("Initialization failed");

    application.connect_activate(gui::initialize);
    application.run(&[]);
}
