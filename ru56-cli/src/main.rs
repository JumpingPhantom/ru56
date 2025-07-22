use ru56_lib::{
    controller::{Controller, Renderer2D},
    force::{self, Force2D},
    particle::Particle2D,
    simulation::Simulator2D,
    vector::Vec2,
};

fn main() {
    let particle = Particle2D::new(Vec2::new(1.0, 1.0), Vec2::zero(), 100.0);

    let f1 = Force2D {
        force_type: force::ForceType::Constant,
        force_vector: Vec2::new(10.0, 0.0),
    };

    let particles = vec![particle];
    let forces = vec![f1];

    let mut sim = Simulator2D::new(particles, forces);
    let mut controller: Controller<Simulator2D, Renderer2D> = Controller::new().simulator(&mut sim);
    controller.execute();
}
