use cursive::view::Nameable;
use cursive::views::TextView;
use cursive::View;
use cursive_tabs::TabPanel;

pub fn gen_view() -> Box<dyn View> {
    Box::new(
        TabPanel::new()
            .with_tab(TextView::new("This is the first view!").with_name("First"))
            .with_tab(TextView::new("This is the second view!").with_name("Second")),
    )
}
