use ru56_lib::{
    force::{self, Force2D},
    particle::Particle2D,
    runner::{Runner2D, Simulation2D},
    vector::Vec2,
};

fn main() {
    let particle = Particle2D::new(Vec2::zero(), Vec2::zero(), 100.0);

    let f1 = Force2D {
        force_type: force::ForceType::Constant,
        force_vector: Vec2::new(10.0, 0.0),
    };

    let f2 = Force2D {
        force_type: force::ForceType::Constant,
        force_vector: -f1.force_vector,
    };

    let particles = vec![particle];
    let forces = vec![f1];

    let mut sim = Simulation2D::new(particles, forces, 0.0);
    let mut runner = Runner2D::new(&mut sim, 100000);

    runner.run();
}
