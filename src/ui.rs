use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

pub fn init(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    let header_bar = gtk::HeaderBar::new();
    let add_tab_button = gtk::Button::new();
    let notebook = gtk::Notebook::new();

    // ########## WIDGET SETUP ##########

    // HeaderBar
    header_bar.set_title(Some("Alacrittab"));
    header_bar.set_show_close_button(true);

    let add_tab_image = gtk::Image::from_icon_name(Some("tab-new-symbolic"), gtk::IconSize::Button);
    add_tab_button.set_image(Some(&add_tab_image));
    header_bar.add(&add_tab_button);

    // ########## CALLBACKS ##########

    // Add tab
    add_tab_button.connect_clicked(clone!(@weak application, @weak notebook => move |_| {
        let widget: gtk::Widget = gtk::GLArea::new().upcast();

        let close_button = gtk::Button::new();
        close_button.set_relief(gtk::ReliefStyle::None);

        let close_image =
            gtk::Image::from_icon_name(Some("window-close-symbolic"), gtk::IconSize::Button);
        close_button.add(&close_image);

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

        close_button.connect_clicked(
            clone!(@weak application, @weak notebook, @weak widget => move |_| {
                let page_num = notebook.page_num(&widget).unwrap();
                notebook.remove_page(Some(page_num));

                match notebook.get_n_pages() {
                    // TODO: This should run when last terminal exits
                    0 => application.quit(),
                    1 => notebook.set_show_tabs(false),
                    _ => {}
                }
            })
        );
    }));

    // ########## FINAL SETUP ##########

    // Open initial tab
    add_tab_button.clicked();

    window.set_default_size(640, 480);
    window.set_titlebar(Some(&header_bar));
    window.add(&notebook);
    window.show_all();
}
