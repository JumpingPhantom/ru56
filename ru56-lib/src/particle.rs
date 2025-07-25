use crate::{
    force::{Force2D, ForceType},
    traits::Object2D,
    vector::Vec2,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Particle2D {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
    forces: Vec<Force2D>,
}

impl Particle2D {
    pub fn new(position: Vec2, velocity: Vec2, mass: f32) -> Self {
        Self {
            position,
            velocity,
            acceleration: Vec2::zero(),
            mass,
            forces: Vec::default(),
        }
    }
}

impl Object2D for Particle2D {
    fn position(&self) -> Vec2 {
        self.position
    }

    fn velocity(&self) -> Vec2 {
        self.velocity
    }

    fn acceleration(&self) -> Vec2 {
        self.acceleration
    }

    fn mass(&self) -> f32 {
        self.mass
    }

    fn forces(&self) -> &Vec<Force2D> {
        &self.forces
    }

    fn exert(&mut self, force: Force2D) {
        self.forces.push(force);
    }

    fn update(&mut self, dt: f32) {
        let mut net_acceleration = Vec2::zero();

        for force in &self.forces {
            let acceleration = force.acceleration(self.mass());
            net_acceleration = net_acceleration + acceleration;

            if force.force_type != ForceType::Constant {
                let idx = self.forces.iter().position(|f| std::ptr::eq(f, force));
                if let Some(idx) = idx {
                    self.forces.remove(idx);
                }
                break;
            }
        }

        self.acceleration = net_acceleration;
        self.velocity = self.velocity + self.acceleration * dt;
        self.position = self.position + self.velocity * dt;
        self.acceleration = Vec2::zero();
    }
}
