use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

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
                Self::add_tab(application, notebook);
            }),
        );
    }

    fn add_tab(application: gtk::Application, notebook: gtk::Notebook) {
        let widget: gtk::Widget = gtk::Label::new(Some("Content")).upcast();

        // Close button
        let close_button = gtk::Button::new();
        close_button.set_relief(gtk::ReliefStyle::None);

        let close_image =
            gtk::Image::from_icon_name(Some("window-close-symbolic"), gtk::IconSize::Button);
        close_button.add(&close_image);

        // Tab
        let label = gtk::Label::new(Some("Terminal"));
        let tab = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        tab.set_center_widget(Some(&label));
        tab.pack_end(&close_button, false, false, 0);
        tab.set_hexpand(true);
        tab.show_all();

        notebook.append_page(&widget, Some(&tab));
        notebook.set_tab_reorderable(&widget, true);
        notebook.show_all();

        if notebook.get_n_pages() == 1 {
            notebook.set_show_tabs(false);
        } else {
            notebook.set_show_tabs(true);
        }

        // Callback
        close_button.connect_clicked(
            clone!(@weak application, @weak notebook, @weak widget => move |_| {
                Self::close_tab(application, notebook, widget);
            }),
        );
    }

    fn close_tab(application: gtk::Application, notebook: gtk::Notebook, widget: gtk::Widget) {
        let page_num = notebook.page_num(&widget).unwrap();
        notebook.remove_page(Some(page_num));

        match notebook.get_n_pages() {
            // TODO: This should run when last terminal exits
            0 => application.quit(),
            1 => notebook.set_show_tabs(false),
            _ => {}
        }
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
