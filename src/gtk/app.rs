use relm4::{
    adw::{self, gtk, prelude::*},
    send, AppUpdate, Model, RelmComponent, Sender, Widgets,
};
use tracing::*;

use crate::gtk::tabs::{TabsModel, TabsMsg};

#[derive(Debug)]
pub enum AppMsg {
    CloseRequest,
}

#[derive(relm4::Components)]
pub struct AppComponents {
    tabs: RelmComponent<TabsModel, AppModel>,
}

#[derive(Default)]
pub struct AppModel {}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

impl AppUpdate for AppModel {
    fn update(
        &mut self,
        msg: Self::Msg,
        _components: &Self::Components,
        _sender: Sender<Self::Msg>,
    ) -> bool {
        debug!(?msg);
        true
    }
}

#[relm4::widget(pub)]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        adw::ApplicationWindow {
            set_default_width: 500,
            set_default_height: 300,
            set_content: main_box = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                append = &gtk::HeaderBar {
                    set_title_widget: Some(&gtk::Label::new(Some("Alacrittab"))),
                    pack_start = &gtk::Button {
                        set_label: "+",
                        connect_clicked[sender = components.tabs.sender()] => move |_| {
                            send!(sender, TabsMsg::AddTab)
                        }
                    }
                },
                append: components.tabs.root_widget()
            },
            connect_close_request(sender) => move |_| {
                send!(sender, AppMsg::CloseRequest);
                gtk::Inhibit(false)
            }
        }
    }
}
