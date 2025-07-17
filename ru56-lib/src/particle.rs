use crate::vector::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle2D {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
}

impl Particle2D {
    pub fn new(position: Vec2, velocity: Vec2, mass: f32) -> Self {
        Self {
            position,
            velocity,
            acceleration: Vec2::zero(),
            mass,
        }
    }
}
