use rustyray::prelude::*;

fn main() {
    let window = Window::new(1280, 720, String::from("Hello from Rust")).vsync(true);
    let rt = OwnedRenderTexture::new(640, 360);

    while !window.should_close() {
        window.draw_render_texture(&rt.0, |d| {
            d.clear(Color::BLACK);
            d.draw_fps(10, 10);
        });
        window.draw(|d| {
            d.clear(Color::BLANK);
            d.draw_render_texture(&rt.0);
        });
    }
}
