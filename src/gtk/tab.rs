use gtk::prelude::*;
use relm::*;
use relm_derive::*;

use crate::gtk::app::AppMsg;

#[derive(Msg)]
pub enum TabMsg {
    Close,
}

pub struct TabParams {
    pub associated_widget: gtk::Widget,
    pub title: String,
    pub stream: StreamHandle<AppMsg>,
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
            TabMsg::Close => self
                .model
                .stream
                .emit(AppMsg::CloseTerminal(self.model.associated_widget.clone())),
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

            clicked => TabMsg::Close
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
