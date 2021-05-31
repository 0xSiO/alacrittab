use std::{collections::HashMap, ops::Deref, sync::Arc};

use alacritty_terminal::config::Config;
use gtk::prelude::*;
use relm::*;
use relm_derive::*;
use tracing::*;

use crate::gtk::{tab::*, terminal::*};

#[derive(Msg, Debug)]
pub enum AppMsg {
    NewTerminal,
    CloseTerminal(gtk::Widget),
    CloseAllTerminals,
    TerminalExit(gtk::Widget),
    GdkEvent(gdk::Event),
    Quit,
}

pub struct AppParams {
    pub config: Config<()>,
}

pub struct AppModel {
    config: Arc<Config<()>>,
    tabs: HashMap<gtk::Widget, Component<Tab>>,
    terminals: HashMap<gtk::Widget, Component<Terminal>>,
    relm: Relm<App>,
}

#[widget]
impl Widget for App {
    fn model(relm: &Relm<Self>, params: AppParams) -> AppModel {
        AppModel {
            config: Arc::new(params.config),
            tabs: Default::default(),
            terminals: Default::default(),
            relm: relm.clone(),
        }
    }

    #[instrument(skip(self))]
    fn update(&mut self, event: AppMsg) {
        use AppMsg::*;

        trace!("received event");
        match event {
            NewTerminal => self.new_terminal(),
            CloseTerminal(widget) => self.close_terminal(widget),
            CloseAllTerminals => self.close_all_terminals(),
            TerminalExit(widget) => self.remove_terminal(widget),
            GdkEvent(event) => match event.get_event_type() {
                gdk::EventType::KeyPress => self.print_key(event.downcast().unwrap()),
                _ => {}
            },
            Quit => gtk::main_quit(),
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
                        clicked => AppMsg::NewTerminal
                    }
                }
            },
            property_default_width: 640,
            property_default_height: 480,

            #[name = "notebook"]
            gtk::Notebook {},

            delete_event(_, _) => (AppMsg::CloseAllTerminals, Inhibit(false)),
        }
    }

    fn init_view(&mut self) {
        self.new_terminal();

        connect!(
            self.model.relm,
            self.widgets.window,
            connect_key_press_event(_, event),
            return (AppMsg::GdkEvent(event.deref().clone()), Inhibit(true))
        );
    }
}

impl App {
    fn new_terminal(&mut self) {
        let terminal = relm::create_component::<Terminal>(TerminalParams {
            config: Arc::clone(&self.model.config),
            stream: self.model.relm.stream().clone(),
        });

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

    fn close_terminal(&mut self, widget: gtk::Widget) {
        if let Some(component) = self.model.terminals.get(&widget) {
            component.emit(TerminalMsg::Quit);
        }
    }

    fn close_all_terminals(&mut self) {
        for terminal in self.model.terminals.values() {
            terminal.emit(TerminalMsg::Quit);
        }
    }

    fn remove_terminal(&mut self, widget: gtk::Widget) {
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

    #[instrument(skip(self, event))]
    fn print_key(&self, event: gdk::EventKey) {
        if let Some(name) = event.get_keyval().name() {
            debug!("key pressed: {}", name);
        }
    }
}
