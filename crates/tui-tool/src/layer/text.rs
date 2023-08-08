#![allow(dead_code)]
use cursive::{views::TextView, View};

pub fn add_view() -> Box<dyn View> {
    Box::new(TextView::new("Hello cursive! Press <q> to quit."))
}
