use rustyray::prelude::*;

fn main() {
    let mut window = WindowBuilder::new(1280, 720, "Hello from Rust")
        .set_config_flags(ConfigFlag::VsyncHint | ConfigFlag::WindowHighdpi)
        .build()
        .unwrap();
    let rt_handle = window.assets.create((640, 360)).unwrap();

    while !window.should_close() {
        if let Some(dt) = window.begin_texture_mode(&rt_handle) {
            dt.clear(Color::BLACK);
            dt.draw_fps(10, 10);
        }

        let d = window.begin_drawing();
        d.clear(Color::BLANK);
        d.draw_render_texture(&rt_handle);
    }
}
