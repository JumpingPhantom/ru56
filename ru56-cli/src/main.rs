use ru56_cli::renderer::Renderer2D;
use ru56_lib::{
    controller::Controller,
    force::{self, Force2D},
    particle::Particle2D,
    simulation::Simulator2D,
    traits::Object2D,
    vector::Vec2,
};

fn main() {
    let mut particle1 = Box::new(Particle2D::new(Vec2::zero(), Vec2::zero(), 100.0));
    let mut particle2 = Box::new(Particle2D::new(Vec2::zero(), Vec2::zero(), 100.0));
    let mut particle3 = Box::new(Particle2D::new(Vec2::zero(), Vec2::zero(), 100.0));

    let f1 = Force2D {
        force_type: force::ForceType::Constant,
        force_vector: Vec2::new(1000.0, 0.0),
    };

    let f2 = Force2D {
        force_type: force::ForceType::Constant,
        force_vector: Vec2::new(-1000.0, 0.0),
    };

    let f3 = Force2D {
        force_type: force::ForceType::Constant,
        force_vector: Vec2::new(1000.0, 1000.0),
    };

    particle1.exert(f1);
    particle2.exert(f2);
    particle3.exert(f3);

    let particles: Vec<Box<dyn Object2D>> = vec![particle1, particle2, particle3];

    let mut sim = Simulator2D::new(particles);
    let renderer = Renderer2D::new();
    let controller: Controller<Simulator2D, Renderer2D> = Controller::new();
    controller.simulator(&mut sim).renderer(renderer).execute();
}
