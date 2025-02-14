use std::env;

use rustyray::prelude::*;

fn main() {
    {
        let exe = env::current_exe().unwrap();
        let exe_dir = exe.parent().unwrap();
        debug_assert!(
            exe_dir.is_dir(),
            "Somehow the parent of the executable is not a directory"
        );
        env::set_current_dir(exe_dir).unwrap();
    }

    let mut window = Window::new(1280, 720, String::from("Hello from Rust")).vsync(true);
    let rt = RenderTexture::new(640, 360);

    while !window.should_close() {
        window.draw_render_texture(&rt, |d| {
            d.clear(Color::BLACK);
            d.draw_fps(&Vector2i::new(10, 10));
        });
        window.draw(|d| {
            d.clear(Color::BLANK);
            d.draw_render_texture(&rt);
        });
    }
}
