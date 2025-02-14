extern crate rustyray;

use rand::Rng;
use rustyray::prelude::*;

struct Bunny {
    pos: Vector2,
    speed: Vector2,
    color: Color,
    dir: Vector2,
}

const MAX_BUNNIES: usize = 50000;

fn main() {
    let mut window = Window::new(1280, 720, String::from("Bunnymark"));
    let tex = OwnedTexture::new(String::from("assets/wabbit_alpha.png"));
    let mut rng = rand::rng();

    let mut bunnies = Vec::<Bunny>::new();
    bunnies.reserve_exact(MAX_BUNNIES);

    while !window.should_close() {
        let dt = window.dt();
        if window.is_mouse_down(MouseButton::MouseButtonLeft) {
            for _ in 0..100 {
                if bunnies.len() < bunnies.capacity() {
                    let mouse_pos = window.get_mouse_pos();
                    bunnies.push(Bunny {
                        pos: mouse_pos,
                        speed: Vector2::new(
                            rng.random_range(1.0..250.0),
                            rng.random_range(1.0..250.0),
                        ),
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

        let screen_size = window.get_screen_size();
        for bunny in &mut bunnies {
            bunny.dir.normalize();
            bunny.pos.x += bunny.speed.x * bunny.dir.x * dt;
            bunny.pos.y += bunny.speed.y * bunny.dir.y * dt;

            let pos_x = bunny.pos.x as i32 + tex.width() / 2;
            let pos_y = bunny.pos.y as i32 + tex.height() / 2;
            if pos_x > screen_size.x || pos_x < 0 {
                bunny.dir.x *= -1.;
            }
            if pos_y > screen_size.y || pos_y - tex.height() < 0 {
                bunny.dir.y *= -1.;
            }
        }

        window.draw(|d| {
            d.clear(Color::WHITE);
            for bunny in &bunnies {
                d.draw_texture(&tex, bunny.pos.x as i32, bunny.pos.y as i32, bunny.color);
            }
            d.draw_rect(
                Rectangle::new(0., 0., screen_size.x as f32, 40.),
                Color::BLACK,
            );
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
