use relm4::{
    adw::{self, gtk},
    factory::{Factory, FactoryPrototype, FactoryVec, FactoryView},
    Sender,
};

use crate::gtk::tabs::TabsMsg;

pub struct Terminal {
    pub term: (),
}

#[derive(Debug)]
pub struct TerminalWidgets {
    gl_area: gtk::GLArea,
}

impl FactoryPrototype for Terminal {
    type Factory = FactoryVec<Self>;
    type Widgets = TerminalWidgets;
    type Root = gtk::GLArea;
    type View = adw::TabView;
    type Msg = TabsMsg;

    fn init_view(
        &self,
        _key: &<Self::Factory as Factory<Self, Self::View>>::Key,
        _sender: Sender<Self::Msg>,
    ) -> Self::Widgets {
        TerminalWidgets {
            gl_area: gtk::GLArea::new(),
        }
    }

    fn position(
        &self,
        _key: &<Self::Factory as Factory<Self, Self::View>>::Key,
    ) -> <Self::View as FactoryView<Self::Root>>::Position {
    }

    fn view(
        &self,
        _key: &<Self::Factory as Factory<Self, Self::View>>::Key,
        _widgets: &Self::Widgets,
    ) {
        // TODO: Redraw?
    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.gl_area
    }
}
