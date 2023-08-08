// use super::text::add_view as text_add_view;
use crate::layer::tabs::add_view as tabs_add_view;

pub fn get_layers() -> Vec<fn(&mut cursive::CursiveRunnable)> {
    vec![/* text_add_view ,*/ tabs_add_view]
}
