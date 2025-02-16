use rustyray::prelude::*;

fn main() {
    let window = Window::new(
        800,
        450,
        String::from("rustyray [audio] example - play sound multiple times"),
    )
    .fps(60)
    .init_audio()
    .unwrap();

    let mut sounds = vec![OwnedSound::new("assets/audio/sound.wav".into())];
    sounds.reserve_exact(10);
    for _ in 1..10 {
        sounds.push(sounds[0].alias())
    }

    let mut current_sound = 0;

    while !window.should_close() {
        if window.is_key_pressed(KeyboardKey::Space) {
            sounds[current_sound].play();
            current_sound += 1;
            if current_sound >= sounds.len() {
                current_sound = 0;
            }
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
        });
    }
}
