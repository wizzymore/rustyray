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

    let music_handle: Handle<Music> = window.assets.load(String::from("assets/audio/country.mp3"));

    while !window.should_close() {
        window.assets.process_assets();

        let restart = window.is_key_pressed(KeyboardKey::Space);
        let toggle_pause = window.is_key_pressed(KeyboardKey::P);

        let time_played = window.assets.get_mut(&music_handle).map(|music| {
            if !music.is_playing() && !music.is_paused() {
                music.play();
            }
            music.update();
            if restart {
                music.restart();
            }
            if toggle_pause {
                music.toggle();
            }
            let length = music.length();
            if length > 0.0 {
                (music.played() / length).min(1.)
            } else {
                0.0
            }
        });

        window.draw(|d| {
            d.clear(Color::RAYWHITE);

            if let Some(time_played) = time_played {
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
            } else {
                const LOADING: &str = "Loading...";
                const SIZE: i32 = 20;
                d.draw_text(
                    LOADING,
                    (SCREEN_WIDTH - d.measure_text(LOADING, SIZE)) / 2,
                    (SCREEN_HEIGHT - SIZE) / 2,
                    SIZE,
                    Color::DARKGRAY,
                );
            }
        });
    }
}
