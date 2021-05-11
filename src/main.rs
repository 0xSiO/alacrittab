use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

mod tabs;

struct Alacrittab<'a> {
    application: &'a gtk::Application,
    widgets: Widgets,
}

struct Widgets {
    window: gtk::ApplicationWindow,
    header_bar: gtk::HeaderBar,
    add_tab_button: gtk::Button,
    notebook: gtk::Notebook,
}

impl<'a> Alacrittab<'a> {
    pub fn new(application: &'a gtk::Application) -> Self {
        let this = Self {
            application,
            widgets: Widgets {
                window: gtk::ApplicationWindow::new(application),
                header_bar: gtk::HeaderBar::new(),
                add_tab_button: gtk::Button::new(),
                notebook: gtk::Notebook::new(),
            },
        };

        this.setup_widgets();
        this.setup_signal_handlers();

        // Open initial tab
        this.widgets.add_tab_button.clicked();

        this
    }

    fn setup_widgets(&self) {
        let Widgets {
            window,
            header_bar,
            add_tab_button,
            notebook,
        } = &self.widgets;

        // Notebook setup
        window.add(notebook);

        // HeaderBar setup
        header_bar.set_title(Some("Alacrittab"));
        header_bar.set_show_close_button(true);

        let add_tab_image =
            gtk::Image::from_icon_name(Some("tab-new-symbolic"), gtk::IconSize::Button);
        add_tab_button.set_image(Some(&add_tab_image));
        header_bar.add(add_tab_button);

        window.set_titlebar(Some(header_bar));

        // Window setup
        window.set_default_size(640, 480);
        window.show_all();
    }

    fn setup_signal_handlers(&self) {
        self.widgets.add_tab_button.connect_clicked(
            clone!(@weak self.application as application, @weak self.widgets.notebook as notebook => move |_| {
                tabs::add_tab(application, notebook);
            }),
        );
    }
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.lucis-fluxum.alacrittab"),
        Default::default(),
    )
    .unwrap();

    application.connect_activate(|application| {
        Alacrittab::new(application);
    });

    application.run(&[]);
}
