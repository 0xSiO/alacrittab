use glib::clone;
use gtk::prelude::*;
use gtk::{IconSize, Orientation, ReliefStyle, Widget};

pub struct Notebook {
    pub inner: gtk::Notebook,
}

impl Notebook {
    pub fn new() -> Self {
        Self {
            inner: gtk::Notebook::new(),
        }
    }

    fn new_tab(title: &str, close_button: &gtk::Button) -> gtk::Box {
        let label = gtk::Label::new(Some(title));
        let tab = gtk::Box::new(Orientation::Horizontal, 0);
        tab.set_center_widget(Some(&label));
        tab.pack_end(close_button, false, false, 0);
        tab.set_hexpand(true);
        tab.show_all();
        tab
    }

    pub fn add_tab(&self, title: &str, widget: impl IsA<Widget>) {
        // Close button
        let close_button = gtk::Button::new();
        close_button.set_relief(ReliefStyle::None);

        let close_image =
            gtk::Image::from_icon_name(Some("window-close-symbolic"), IconSize::Button);
        close_button.add(&close_image);

        // Tab
        let tab = Self::new_tab(title, &close_button);
        let page_num = self.inner.append_page(&widget, Some(&tab));
        self.inner.set_tab_reorderable(&widget, true);

        // Callbacks
        close_button.connect_enter_notify_event(clone!(@strong close_image => move |_, _| {
            close_image.set_from_icon_name(Some("window-close"), IconSize::Button);
            Inhibit(false)
        }));

        close_button.connect_leave_notify_event(clone!(@strong close_image => move |_, _| {
            close_image.set_from_icon_name(Some("window-close-symbolic"), IconSize::Button);
            Inhibit(false)
        }));

        close_button.connect_clicked(clone!(@strong self.inner as notebook => move |_| {
            notebook.remove_page(Some(page_num));
        }));
    }
}
