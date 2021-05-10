use crate::notebook::Notebook;

pub struct AppState {}

pub struct GuiState {
    pub application: gtk::Application,
    pub window: gtk::ApplicationWindow,
    pub header_bar: gtk::HeaderBar,
    pub notebook: Notebook,
}
