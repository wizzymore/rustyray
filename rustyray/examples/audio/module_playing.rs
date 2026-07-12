use rand::{Rng, rngs::ThreadRng};
use rustyray::prelude::*;

struct CircleWave {
    pos: Vector2,
    radius: f32,
    alpha: f32,
    speed: f32,
    color: Color,
}

const SCREEN_HEIGHT: i32 = 450;
const SCREEN_WIDTH: i32 = 800;

const MAX_CIRCLES: usize = 64;
const COLORS: &[Color] = &[
    Color::ORANGE,
    Color::RED,
    Color::GOLD,
    Color::LIME,
    Color::BLUE,
    Color::VIOLET,
    Color::BROWN,
    Color::LIGHTGRAY,
    Color::PINK,
    Color::YELLOW,
    Color::GREEN,
    Color::SKYBLUE,
    Color::PURPLE,
    Color::BEIGE,
];

fn create_circle(rng: &mut ThreadRng) -> CircleWave {
    let radius = rng.random_range(10.0..40.0);
    CircleWave {
        pos: Vector2 {
            x: rng.random_range(radius..(800. - radius)),
            y: rng.random_range(radius..(800. - radius)),
        },
        radius,
        alpha: 0.,
        speed: rng.random_range(1.0..100.0) / 2000.,
        color: COLORS[rng.random_range(0..COLORS.len())],
    }
}

fn main() {
    let mut window = WindowBuilder::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "rustyray [audio] example - module playing (streaming)",
    )
    .set_config_flags(ConfigFlag::WindowHighdpi)
    .set_fps(60)
    .init_audio()
    .build()
    .unwrap();

    let mut rng = rand::rng();

    let mut circles = Vec::<CircleWave>::new();
    circles.reserve_exact(MAX_CIRCLES);

    for _ in 0..MAX_CIRCLES {
        circles.push(create_circle(&mut rng));
    }

    let music_handle: Handle<Music> = window.assets.load(String::from("assets/audio/mini1111.xm"));

    let mut pitch = 1.;

    while !window.should_close() {
        window.assets.process_assets();

        let restart = window.is_key_pressed(KeyboardKey::Space);
        let toggle_pause = window.is_key_pressed(KeyboardKey::P);
        if window.is_key_down(KeyboardKey::Down) {
            pitch -= 0.01;
        } else if window.is_key_down(KeyboardKey::Up) {
            pitch += 0.01;
        }

        let frame = window.assets.get_mut(&music_handle).map(|music| {
            if !music.is_playing() && !music.is_paused() {
                music.looping(false);
                music.play();
            }
            music.update();
            if restart {
                music.restart();
            }
            if toggle_pause {
                music.toggle();
            }
            music.pitch(pitch);
            let length = music.length();
            let time_played = if length > 0.0 {
                music.played() / length * (800. - 40.)
            } else {
                0.0
            };
            let playing = music.is_playing();
            (time_played, playing)
        });

        if let Some((_, playing)) = frame {
            if playing {
                circles.iter_mut().for_each(|circle| {
                    circle.alpha += circle.speed;
                    circle.radius += circle.speed * 10.;

                    if circle.alpha > 1. {
                        circle.speed *= -1.;
                    }

                    if circle.alpha <= 0. {
                        *circle = create_circle(&mut rng);
                    }

                    circle.color.fade(circle.alpha);
                });
            }
        }

        window.draw(|d| {
            d.clear(Color::RAYWHITE);

            if let Some((time_played, _)) = frame {
                circles.iter().for_each(|circle| {
                    d.draw_circle(circle.pos, circle.radius, circle.color);
                });

                d.draw_rect(
                    Rectangle::new(
                        20.,
                        SCREEN_HEIGHT as f32 - 20. - 12.,
                        SCREEN_WIDTH as f32 - 40.,
                        12.,
                    ),
                    Color::LIGHTGRAY,
                );
                d.draw_rect(
                    Rectangle::new(20., SCREEN_HEIGHT as f32 - 20. - 12., time_played, 12.),
                    Color::MAROON,
                );
                d.draw_rect_lines(
                    Rectangle::new(
                        20.,
                        SCREEN_HEIGHT as f32 - 20. - 12.,
                        SCREEN_WIDTH as f32 - 40.,
                        12.,
                    ),
                    Color::GRAY,
                );

                d.draw_rect(Rectangle::new(20., 20., 425., 145.), Color::WHITE);
                d.draw_rect_lines(Rectangle::new(20., 20., 425., 145.), Color::GRAY);
                d.draw_text("PRESS SPACE TO RESTART MUSIC", 40, 40, 20, Color::BLACK);
                d.draw_text("PRESS P TO PAUSE/RESUME", 40, 70, 20, Color::BLACK);
                d.draw_text("PRESS UP/DOWN TO CHANGE SPEED", 40, 100, 20, Color::BLACK);
                d.draw_text(format!("SPEED: {}", pitch), 40, 130, 20, Color::MAROON);
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