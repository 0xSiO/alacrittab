mod display;
mod event_handler;

pub use self::{display::Display, event_handler::EventHandler};

// TODO: Methods needed for rendering
pub trait Render {}

// TODO: Temporary impls while setting things up
impl Render for () {}
