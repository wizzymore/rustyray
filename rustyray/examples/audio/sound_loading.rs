use rustyray::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

fn main() {
    let mut window = WindowBuilder::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        String::from("rustyray [audio] example - sound loading and playing"),
    )
    .set_config_flags(ConfigFlag::WindowHighdpi)
    .set_fps(60)
    .init_audio()
    .build()
    .unwrap();

    let fx_wav: Handle<Sound> = window.assets.load(String::from("assets/audio/sound.wav"));
    let fx_ogg: Handle<Sound> = window.assets.load(String::from("assets/audio/target.ogg"));

    while !window.should_close() {
        window.assets.process_assets();

        let ready = window.assets.is_ready(&fx_wav) && window.assets.is_ready(&fx_ogg);

        if ready {
            if window.is_key_pressed(KeyboardKey::Space) {
                if let Some(fx) = window.assets.get(&fx_wav) {
                    fx.play();
                }
            }
            if window.is_key_pressed(KeyboardKey::Enter) {
                if let Some(fx) = window.assets.get(&fx_ogg) {
                    fx.play();
                }
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

                d.draw_text(
                    "Press ENTER to PLAY the OGG sound!",
                    200,
                    220,
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
