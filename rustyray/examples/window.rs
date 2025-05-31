use rustyray::prelude::*;

fn main() {
    let window = WindowBuilder::new(1280, 720, "Hello from Rust")
        .set_config_flags(ConfigFlag::VsyncHint | ConfigFlag::WindowHighdpi)
        .build()
        .unwrap();
    let rt = OwnedRenderTexture::new(640, 360).unwrap();

    while !window.should_close() {
        window.draw_render_texture(&rt, |d| {
            d.clear(Color::BLACK);
            d.draw_fps(10, 10);
        });
        window
            .draw(|d| {
                d.clear(Color::BLANK);
                d.draw_render_texture(&rt);
            })
            .unwrap();
    }
}
