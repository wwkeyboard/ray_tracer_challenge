mod canvas;
mod color;
mod matrix;
mod point;
use canvas::Canvas;
use point::Tuple;
use std::env;

use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let grav: f32 = args[1].parse().unwrap();
    let wind: f32 = args[2].parse().unwrap();

    let world = World {
        gravity: Tuple::vector(0., 0., -1.0 * grav),
        wind: Tuple::vector(0., -1.0 * wind, 0.),
    };

    let mut p = Projectile {
        position: Tuple::point(0., 0., 1.),
        velocity: Tuple::vector(0., 6., 6.),
    };

    let mut canvas = Canvas::new(500, 200);

    println!("Starting Simulation");

    while p.position.z > 0. {
        tick(&world, &mut p);

        canvas.safe_write_pixel(
            (p.position.y) as usize,
            (200. - p.position.z) as usize,
            color::Color::new(1., 1., 1.),
        );
    }

    let p = Path::new("./out.ppm");
    canvas.save_ppm(p);
}

pub struct World {
    gravity: Tuple,
    wind: Tuple,
}

#[derive(Copy, Clone)]
pub struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

fn tick(world: &World, p: &mut Projectile) {
    p.position = p.position + p.velocity;
    p.velocity = p.velocity + world.gravity + world.wind;
}
