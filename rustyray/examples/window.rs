use rustyray::prelude::*;

fn main() {
    let mut window = WindowBuilder::new(1280, 720, "Hello from Rust")
        .set_config_flags(ConfigFlag::VsyncHint | ConfigFlag::WindowHighdpi)
        .build()
        .unwrap();
    let rt = OwnedRenderTexture::new(640, 360).unwrap();

    while !window.should_close() {
        {
            let dt = window.begin_texture_mode(&rt);
            dt.clear(Color::BLACK);
            dt.draw_fps(10, 10);
        }

        let d = window.begin_drawing();
        d.clear(Color::BLANK);
        d.draw_render_texture(&rt);
    }
}
