use rustyray::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

fn main() {
    let mut window = WindowBuilder::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "rustyray [audio] example - music playing (streaming)",
    )
    .set_config_flags(ConfigFlag::WindowHighdpi)
    .set_fps(60)
    .init_audio()
    .build()
    .unwrap();

    let mut music = OwnedMusic::new("assets/audio/country.mp3".into());

    music.play();

    while !window.should_close() {
        music.update();

        if window.is_key_pressed(KeyboardKey::Space) {
            music.restart()
        }

        if window.is_key_pressed(KeyboardKey::P) {
            music.toggle();
        }

        let time_played = (music.played() / music.length()).min(1.);

        window.draw(|d| {
            d.clear(Color::RAYWHITE);

            d.draw_text("MUSIC SHOULD BE PLAYING!", 255, 150, 20, Color::DARKGRAY);

            d.draw_rect(Rectangle::new(200., 200., 400., 12.), Color::LIGHTGRAY);
            d.draw_rect(
                Rectangle::new(200., 200., time_played * 400.0, 12.),
                Color::MAROON,
            );
            d.draw_rect_lines(Rectangle::new(200., 200., 400., 12.), Color::GRAY);

            d.draw_text(
                "PRESS SPACE TO RESTART MUSIC",
                215,
                250,
                20,
                Color::DARKGRAY,
            );
            d.draw_text(
                "PRESS P TO PAUSE/RESUME MUSIC",
                208,
                280,
                20,
                Color::DARKGRAY,
            );
        });
    }
}
