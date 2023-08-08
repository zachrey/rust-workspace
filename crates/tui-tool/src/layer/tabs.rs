use cursive::view::Nameable;
use cursive::views::TextView;
use cursive::CursiveRunnable;
use cursive_tabs::TabPanel;

pub fn add_view(siv: &mut CursiveRunnable) {
    siv.add_layer(
        TabPanel::new()
            .with_tab(TextView::new("This is the first view!").with_name("First"))
            .with_tab(TextView::new("This is the second view!").with_name("Second")),
    );
}
