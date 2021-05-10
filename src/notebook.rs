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

    pub fn add_tab(&self, title: &str, widget: impl IsA<Widget>) {
        // Close button
        let button = gtk::Button::new();
        button.set_relief(ReliefStyle::None);

        let close_image =
            gtk::Image::from_icon_name(Some("window-close-symbolic"), IconSize::Button);
        button.add(&close_image);

        // Label
        let label = gtk::Label::new(Some(title));

        // Tab
        let tab = gtk::Box::new(Orientation::Horizontal, 0);
        tab.set_center_widget(Some(&label));
        tab.pack_end(&button, false, false, 0);
        tab.set_hexpand(true);
        tab.show_all();

        self.inner.append_page(&widget, Some(&tab));
        self.inner.set_tab_reorderable(&widget, true);

        // Callbacks
        button.connect_enter_notify_event(
            glib::clone!(@weak close_image => @default-return Inhibit(false), move |_, _| {
                close_image.set_from_icon_name(Some("window-close"), IconSize::Button);
                Inhibit(false)
            }),
        );

        button.connect_leave_notify_event(
            glib::clone!(@weak close_image => @default-return Inhibit(false), move |_, _| {
                close_image.set_from_icon_name(Some("window-close-symbolic"), IconSize::Button);
                Inhibit(false)
            }),
        );

        button.connect_clicked(glib::clone!(@weak self.inner as notebook => move |_| {
            let index = notebook
                .page_num(&widget)
                .unwrap();
            notebook.remove_page(Some(index));
        }));
    }
}
