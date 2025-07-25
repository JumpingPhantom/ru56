use crate::particle::Particle2D;

pub trait Renderer {
    fn render(&self, data: &Vec<Particle2D>);
}

pub trait Simulator {
    fn update(&mut self, dt: f32) -> Option<&Vec<Particle2D>>;
}