use rustyray::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

fn main() {
    let mut window = WindowBuilder::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "rustyray [audio] example - play sound multiple times",
    )
    .set_config_flags(ConfigFlag::WindowHighdpi)
    .set_fps(60)
    .init_audio()
    .build()
    .unwrap();

    let sound: Handle<Sound> = window.assets.load(String::from("assets/audio/sound.wav"));

    let mut sounds: Vec<Handle<Sound>> = Vec::new();
    let mut current_sound = 0;

    while !window.should_close() {
        window.assets.process_assets();

        if sounds.is_empty() && window.assets.is_ready(&sound) {
            sounds.push(sound.clone());
            sounds.reserve_exact(10);
            for _ in 1..10 {
                sounds.push(Sound::alias(&mut window.assets, &sound).unwrap());
            }
        }

        let ready = !sounds.is_empty();

        if ready && window.is_key_pressed(KeyboardKey::Space) {
            if let Some(fx) = window.assets.get(&sounds[current_sound]) {
                fx.play();
            }
            current_sound += 1;
            if current_sound >= sounds.len() {
                current_sound = 0;
            }
        }

        window.draw(|d| {
            d.clear(Color::RAYWHITE);

            if ready {
                d.draw_text(
                    "Press SPACE to PLAY the WAV sound!",
                    200,
                    180,
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
