use relm4::{
    adw::{self, gtk, prelude::*},
    factory::FactoryVec,
    send, ComponentUpdate, Model, Sender, Widgets,
};
use tracing::*;

use crate::gtk::{app::AppModel, terminal::Terminal};

#[derive(Debug)]
pub enum TabsMsg {
    AddTab,
    RemoveTab,
}

pub struct TabsModel {
    terminals: FactoryVec<Terminal>,
}

impl Model for TabsModel {
    type Msg = TabsMsg;
    type Widgets = TabsWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for TabsModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        Self {
            terminals: FactoryVec::new(),
        }
    }

    fn update(
        &mut self,
        msg: Self::Msg,
        _components: &Self::Components,
        _sender: Sender<Self::Msg>,
        _parent_sender: Sender<<AppModel as Model>::Msg>,
    ) {
        debug!(?msg);
        match msg {
            TabsMsg::AddTab => self.terminals.push(Terminal { term: () }),
            // TODO: Use dynamic index for removal
            TabsMsg::RemoveTab => {
                self.terminals.pop();
            }
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<TabsModel, AppModel> for TabsWidgets {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            append: tab_bar = &adw::TabBar {
                set_view = Some(&adw::TabView) {
                    factory!(model.terminals),
                    connect_close_page(sender) => move |_, _| {
                        send!(sender, TabsMsg::RemoveTab);
                        true
                    }
                }
            },
            append: &tab_bar.view().unwrap()
        }
    }

    fn pre_init() {
        send!(sender, TabsMsg::AddTab);
    }
}
