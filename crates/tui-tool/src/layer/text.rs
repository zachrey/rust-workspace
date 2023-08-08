#![allow(dead_code)]
use cursive::{views::TextView, CursiveRunnable};

pub fn add_view(siv: &mut CursiveRunnable) {
    siv.add_layer(TextView::new("Hello cursive! Press <q> to quit."));
}
