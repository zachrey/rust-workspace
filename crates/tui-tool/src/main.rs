mod layer;
mod tui_instance;

fn main() {
    let mut siv = tui_instance::create_tui_instance();

    siv.run();
}
