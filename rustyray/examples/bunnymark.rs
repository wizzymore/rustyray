extern crate rustyray;

use rand::RngExt;
use rustyray::prelude::*;

struct Bunny {
    pos: Vector2,
    speed: f32,
    color: Color,
    dir: Vector2,
}

const MAX_BUNNIES: usize = 50000;

fn main() {
    let mut window = WindowBuilder::new(1280, 720, "Bunnymark")
        .set_config_flags(ConfigFlag::WindowHighdpi)
        .build()
        .unwrap();
    let tex_handle = window.assets.load(String::from("assets/wabbit_alpha.png"));
    let tex_size = Vector2i { x: 26, y: 32 };
    let mut rng = rand::rng();

    let mut bunnies = Vec::<Bunny>::new();
    bunnies.reserve_exact(MAX_BUNNIES);
    let screen_size = window.screen_size();
    let bar_rect = Rectangle::new(0., 0., screen_size.x as f32, 40.);

    while !window.should_close() {
        let dt = window.frame_time();
        if window.is_mouse_down(MouseButton::Left) {
            for _ in 0..100 {
                if bunnies.len() < bunnies.capacity() {
                    let mouse_pos = window.mouse_pos();
                    bunnies.push(Bunny {
                        pos: mouse_pos,
                        speed: rng.random_range(1.0..250.0),
                        color: Color::new(
                            rng.random_range(50..240),
                            rng.random_range(80..240),
                            rng.random_range(100..240),
                            255,
                        ),
                        dir: Vector2::new(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0)),
                    });
                }
            }
        }

        bunnies.iter_mut().for_each(|bunny| {
            bunny.dir.normalize();
            bunny.pos += bunny.speed * bunny.dir * dt;

            let pos = bunny.pos.to_vector2i() + tex_size / 2;
            if pos.x > screen_size.x || pos.x < 0 {
                bunny.dir.x *= -1.;
            }
            if pos.y > screen_size.y || pos.y - tex_size.y < 0 {
                bunny.dir.y *= -1.;
            }
        });

        window.draw(|d| {
            d.clear(Color::WHITE);

            bunnies.iter().for_each(|bunny| {
                d.draw_texture(
                    &tex_handle,
                    bunny.pos.x as i32,
                    bunny.pos.y as i32,
                    bunny.color,
                );
            });

            d.draw_rect(bar_rect, Color::BLACK);
            d.draw_text(
                format!("bunnies: {}", bunnies.len()),
                120,
                10,
                20,
                Color::GREEN,
            );
            d.draw_text(
                format!("batched draw calls: {}", 1 + bunnies.len() / MAX_BUNNIES),
                320,
                10,
                20,
                Color::MAROON,
            );
            d.draw_fps(10, 10);
        });
    }
}
