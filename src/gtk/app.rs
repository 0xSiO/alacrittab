use std::{collections::HashMap, io::Write, ops::Deref};

use gtk::prelude::*;
use relm::*;
use relm_derive::*;

use crate::gtk::{tab::*, terminal::*};

pub struct AppModel {
    tabs: HashMap<gtk::Widget, Component<Tab>>,
    terminals: HashMap<gtk::Widget, Component<Terminal>>,
    relm: Relm<App>,
}

#[derive(Msg)]
pub enum AppMsg {
    NewTab,
    CloseTab(gtk::Widget),
    GdkEvent(gdk::Event),
    Quit,
}

#[widget]
impl Widget for App {
    fn model(relm: &Relm<Self>, _param: ()) -> AppModel {
        AppModel {
            tabs: Default::default(),
            terminals: Default::default(),
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: AppMsg) {
        match event {
            AppMsg::NewTab => self.add_tab(),
            AppMsg::CloseTab(widget) => self.remove_tab(widget),
            AppMsg::GdkEvent(event) => match event.get_event_type() {
                gdk::EventType::KeyPress => self.print_key(event.downcast().unwrap()),
                _ => {}
            },
            AppMsg::Quit => gtk::main_quit(),
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

        connect!(
            self.model.relm,
            self.widgets.window,
            connect_key_press_event(_, event),
            return (AppMsg::GdkEvent(event.deref().clone()), Inhibit(true))
        );
    }
}

impl App {
    fn add_tab(&mut self) {
        let terminal = relm::create_component::<Terminal>(TerminalParams {});
        let widget: gtk::Widget = terminal.widget().clone().upcast();
        let notebook: &gtk::Notebook = &self.widgets.notebook;

        let page_num = notebook.append_page(&widget, Option::<&gtk::Box>::None);
        let tab_component = relm::create_component::<Tab>(TabParams {
            associated_widget: widget.clone(),
            title: format!("Tab {}", page_num),
            stream: self.model.relm.stream().clone(),
        });
        notebook.set_tab_label(&widget, Some(tab_component.widget()));
        notebook.set_tab_reorderable(&widget, true);
        self.model.tabs.insert(widget.clone(), tab_component);
        self.model.terminals.insert(widget, terminal);

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
        self.model.terminals.remove(&widget);

        match self.widgets.notebook.get_n_pages() {
            0 => self.model.relm.stream().emit(AppMsg::Quit),
            1 => self.widgets.notebook.set_show_tabs(false),
            _ => {}
        }
    }

    fn print_key(&self, event: gdk::EventKey) {
        event.get_keyval().to_unicode().map(|c| print!("{}", c));
        std::io::stdout().flush().unwrap();
    }
}
