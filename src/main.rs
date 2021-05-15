use gio::prelude::*;

mod ui;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.lucis-fluxum.alacrittab"),
        Default::default(),
    )
    .unwrap();

    application.connect_activate(ui::init);
    application.run(&[]);
}
