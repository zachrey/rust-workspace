use cursive::View;

// use super::text::add_view as text_add_view;
use crate::layer::tabs::gen_view as tabs_gen_view;

pub fn get_layers() -> Vec<Box<dyn View>> {
    vec![/* text_add_view ,*/ tabs_gen_view()]
}
