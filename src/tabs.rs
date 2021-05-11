use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

pub fn add_tab(application: gtk::Application, notebook: gtk::Notebook) {
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
            close_tab(application, notebook, widget);
        }),
    );
}

pub fn close_tab(application: gtk::Application, notebook: gtk::Notebook, widget: gtk::Widget) {
    let page_num = notebook.page_num(&widget).unwrap();
    notebook.remove_page(Some(page_num));

    match notebook.get_n_pages() {
        // TODO: This should run when last terminal exits
        0 => application.quit(),
        1 => notebook.set_show_tabs(false),
        _ => {}
    }
}
