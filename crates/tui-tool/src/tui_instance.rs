use cursive::CursiveRunnable;

use crate::layer::layers::get_layers;

pub fn create_tui_instance() -> CursiveRunnable {
    let mut siv = cursive::default();
    // 配置 app 基础能力
    config_global(&mut siv);

    // 所有的 layer
    config_layer(&mut siv);

    siv
}

fn config_global(siv: &mut CursiveRunnable) {
    siv.add_global_callback('q', |s| s.quit());
}

fn config_layer(siv: &mut CursiveRunnable) {
    for v in get_layers().into_iter() {
        siv.add_layer(v);
    }
}
