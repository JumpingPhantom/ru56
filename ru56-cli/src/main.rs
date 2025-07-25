use ru56_cli::renderer::Renderer2D;
use ru56_lib::{
    controller::Controller,
    force::{self, Force2D},
    particle::Particle2D,
    simulation::Simulator2D,
    vector::Vec2,
};

fn main() {
    let particle1 = Particle2D::new(Vec2::new(1.0, 0.0), Vec2::zero(), 100.0);
    let particle2 = Particle2D::new(Vec2::new(0.0, 1.0), Vec2::zero(), 100.0);
    let particle3 = Particle2D::new(Vec2::new(0.8, 0.0), Vec2::zero(), 100.0);
    let particle4 = Particle2D::new(Vec2::new(0.3, 3.0), Vec2::zero(), 100.0);

    let f1 = Force2D {
        force_type: force::ForceType::Constant,
        force_vector: Vec2::new(1000.0, -5000.5),
    };
    let f3 = Force2D {
        force_type: force::ForceType::Constant,
        force_vector: Vec2::new(10.0, 5000.5),
    };
    let f2 = Force2D {
        force_type: force::ForceType::Constant,
        force_vector: Vec2::new(0.3, -100.5),
    };

    let particles = vec![particle1, particle2, particle3, particle4];
    let forces: Vec<Force2D> = vec![f1, f2, f3];

    let mut sim = Simulator2D::new(particles, forces);
    let renderer = Renderer2D::new();
    let controller: Controller<Simulator2D, Renderer2D> = Controller::new();
    controller.simulator(&mut sim).renderer(renderer).execute();
}
