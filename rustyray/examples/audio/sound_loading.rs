use rustyray::prelude::*;

fn main() {
    let window = Window::new(
        800,
        450,
        String::from("rustyray [audio] example - sound loading and playing"),
    )
    .fps(60)
    .init_audio()
    .unwrap();

    let fx_wav = OwnedSound::new("assets/audio/sound.wav".into());
    let fx_ogg = OwnedSound::new("assets/audio/target.ogg".into());

    while !window.should_close() {
        if window.is_key_pressed(KeyboardKey::Space) {
            fx_wav.play();
        }
        if window.is_key_pressed(KeyboardKey::Enter) {
            fx_ogg.play();
        }

        window.draw(|d| {
            d.clear(Color::RAYWHITE);

            d.draw_text(
                "Press SPACE to PLAY the WAV sound!".into(),
                200,
                180,
                20,
                Color::DARKGRAY,
            );

            d.draw_text(
                "Press ENTER to PLAY the OGG sound!".into(),
                200,
                220,
                20,
                Color::DARKGRAY,
            );
        });
    }
}
