use std::collections::HashMap;

use gtk::prelude::*;
use relm::*;
use relm_derive::*;

#[derive(Msg)]
pub enum TabMsg {
    CloseClicked,
}

pub struct TabParams {
    associated_widget: gtk::Widget,
    title: String,
    stream: StreamHandle<AppMsg>,
}

pub struct TabModel {
    associated_widget: gtk::Widget,
    title: String,
    stream: StreamHandle<AppMsg>,
}

#[widget]
impl Widget for Tab {
    fn model(_relm: &Relm<Self>, params: TabParams) -> TabModel {
        TabModel {
            associated_widget: params.associated_widget,
            title: params.title,
            stream: params.stream,
        }
    }

    fn update(&mut self, event: TabMsg) {
        match event {
            TabMsg::CloseClicked => self
                .model
                .stream
                .emit(AppMsg::CloseTab(self.model.associated_widget.clone())),
        }
    }

    view! {
        #[name = "container"]
        gtk::Box {
            orientation: gtk::Orientation::Horizontal,
            hexpand: true,
            center_widget: view! {
                gtk::Label {
                    text: &self.model.title
                }
            }
        }

        #[name = "close_button"]
        gtk::Button {
            relief: gtk::ReliefStyle::None,

            #[name = "close_image"]
            gtk::Image {},

            clicked => TabMsg::CloseClicked
        }
    }

    fn init_view(&mut self) {
        let close_image: &gtk::Image = &self.widgets.close_image;
        close_image.set_from_icon_name(Some("window-close-symbolic"), gtk::IconSize::Button);
        self.widgets
            .container
            .pack_end(&self.widgets.close_button, false, false, 0);
        self.widgets.container.show_all();
    }
}

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

fn main() {
    App::run(()).unwrap();
}
