use std::collections::HashMap;

use gtk::prelude::*;
use relm::*;
use relm_derive::*;

use crate::tab::*;

pub struct AppModel {
    tabs: HashMap<gtk::Widget, Component<Tab>>,
    relm: Relm<App>,
}

#[derive(Msg)]
pub enum AppMsg {
    NewTab,
    CloseTab(gtk::Widget),
    Quit,
}

#[widget]
impl Widget for App {
    fn model(relm: &Relm<Self>, _param: ()) -> AppModel {
        AppModel {
            tabs: Default::default(),
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: AppMsg) {
        match event {
            AppMsg::Quit => gtk::main_quit(),
            AppMsg::NewTab => self.add_tab(),
            AppMsg::CloseTab(widget) => self.remove_tab(widget),
        }
    }

    view! {
        #[name = "window"]
        gtk::ApplicationWindow {
            titlebar: view! {
                gtk::HeaderBar {
                    title: Some("Alacrittab"),
                    show_close_button: true,

                    #[name = "create_tab_button"]
                    gtk::Button {
                        gtk::Label {
                            text: "+"
                        },
                        clicked => AppMsg::NewTab
                    }
                }
            },
            property_default_width: 640,
            property_default_height: 480,

            #[name = "notebook"]
            gtk::Notebook {},

            delete_event(_, _) => (AppMsg::Quit, Inhibit(false)),
        }
    }

    fn init_view(&mut self) {
        self.add_tab();
    }
}

impl App {
    fn add_tab(&mut self) {
        let widget: gtk::Widget = gtk::GLArea::new().upcast();
        let notebook: &gtk::Notebook = &self.widgets.notebook;

        let page_num = notebook.append_page(&widget, Option::<&gtk::Box>::None);
        let tab_component = relm::create_component::<Tab>(TabParams {
            associated_widget: widget.clone(),
            title: format!("Tab {}", page_num),
            stream: self.model.relm.stream().clone(),
        });
        notebook.set_tab_label(&widget, Some(tab_component.widget()));
        notebook.set_tab_reorderable(&widget, true);
        self.model.tabs.insert(widget, tab_component);

        notebook.show_all();

        if notebook.get_n_pages() == 1 {
            notebook.set_show_tabs(false);
        } else {
            notebook.set_show_tabs(true);
        }
    }

    fn remove_tab(&mut self, widget: gtk::Widget) {
        let page_num = self.widgets.notebook.page_num(&widget);
        self.widgets.notebook.remove_page(page_num);
        self.model.tabs.remove(&widget);

        match self.widgets.notebook.get_n_pages() {
            0 => self.model.relm.stream().emit(AppMsg::Quit),
            1 => self.widgets.notebook.set_show_tabs(false),
            _ => {}
        }
    }
}
