use rand::{rngs::ThreadRng, Rng};
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
    let window = WindowBuilder::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "rustyray [audio] example - module playing (streaming)",
    )
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

    let mut music = OwnedMusic::new("assets/audio/mini1111.xm".into());
    music.looping(false);
    let mut pitch = 1.;

    music.play();

    while !window.should_close() {
        music.update();

        if window.is_key_pressed(KeyboardKey::Space) {
            music.restart()
        }

        if window.is_key_pressed(KeyboardKey::P) {
            music.toggle();
        }

        if window.is_key_down(KeyboardKey::Down) {
            pitch -= 0.01;
        } else if window.is_key_down(KeyboardKey::Up) {
            pitch += 0.01
        }

        music.pitch(pitch);

        let time_played = music.played() / music.length() * (800. - 40.);

        if music.is_playing() {
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

        window
            .draw(|d| {
                d.clear(Color::RAYWHITE);

                circles.iter().for_each(|circle| {
                    d.draw_circle(circle.pos, circle.radius, circle.color);
                });

                // Draw time bar
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

                // Draw help instructions
                d.draw_rect(Rectangle::new(20., 20., 425., 145.), Color::WHITE);
                d.draw_rect_lines(Rectangle::new(20., 20., 425., 145.), Color::GRAY);
                d.draw_text(
                    "PRESS SPACE TO RESTART MUSIC".into(),
                    40,
                    40,
                    20,
                    Color::BLACK,
                );
                d.draw_text("PRESS P TO PAUSE/RESUME".into(), 40, 70, 20, Color::BLACK);
                d.draw_text(
                    "PRESS UP/DOWN TO CHANGE SPEED".into(),
                    40,
                    100,
                    20,
                    Color::BLACK,
                );
                d.draw_text(format!("SPEED: {}", pitch), 40, 130, 20, Color::MAROON);
            })
            .unwrap();
    }
}
