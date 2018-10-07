mod point;

use point::Tuple;

fn main() {
    let world = World {
        gravity: Tuple::vector(0., 0., -1.),
        wind: Tuple::vector(0., 0., 0.),
    };

    let mut p = Projectile {
        position: Tuple::point(0., 0., 1.),
        velocity: Tuple::vector(0., 5., 4.),
    };

    println!("Starting Simulation");

    while p.position.z > 0. {
        tick(&world, &mut p);
        println!("Position {:?}  Velocity {:?}", p.position, p.velocity);
    }
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
    p.velocity = p.velocity + world.gravity;
}
