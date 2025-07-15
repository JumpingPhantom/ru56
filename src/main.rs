mod force;
mod particle;
mod runner;
mod vector;

use crate::force::Force2D;
use crate::runner::{Runner2D, Simulation2D};

use crate::{particle::Particle2D, vector::Vec2};

fn main() {
    let particle = Particle2D::new(Vec2::zero(), Vec2::zero(), 100.0);

    let f1 = Force2D {
        force_type: force::ForceType::Constant,
        force_vector: Vec2::new(0.0, -9.8),
    }; 

    let f2 = Force2D {
        force_type: force::ForceType::Constant,
        force_vector: -f1.force_vector
    };

    let particles = vec![particle];
    let forces = vec![f1, f2];

    let mut sim = Simulation2D::new(particles, forces, 0.0);
    let mut runner = Runner2D::new(&mut sim, 100000);

    runner.run();
}
