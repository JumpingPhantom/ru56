use crate::{force::Force2D, particle::Particle2D, vector::Vec2};

pub trait Renderer {
    fn render(&self, data: &Vec<Box<dyn Object2D>>);
}

pub trait Simulator {
    fn update(&mut self, dt: f32) -> Option<&Vec<Particle2D>>;
}

pub trait Object2D {
    fn position(&self) -> Vec2;
    fn velocity(&self) -> Vec2;
    fn acceleration(&self) -> Vec2;
    fn forces(&self) -> &Vec<Force2D>;
    fn mass(&self) -> f32;
    fn exert(&mut self, force: Force2D);
    fn update(&mut self, dt: f32);
}
