use crate::{
    force::{Force2D, ForceType},
    particle::Particle2D,
    vector::Vec2,
};

pub trait Simulator {
    fn update(&mut self, dt: f32);
}

pub struct Simulator2D {
    pub particles: Vec<Particle2D>,
    pub forces: Vec<Force2D>,
}

impl Simulator2D {
    pub fn new(particles: Vec<Particle2D>, forces: Vec<Force2D>) -> Self {
        Self {
            particles,
            forces,
        }
    }
}

impl Simulator for Simulator2D {
    fn update(&mut self, dt: f32) {
        if self.particles.is_empty() {
            return;
        }

        let mut accelerations = vec![Vec2::zero(); self.particles.len()];

        for (i, particle) in self.particles.iter().enumerate() {
            let mut total_acceleration = Vec2::zero();
            for force in &self.forces {
                let acceleration = force.acceleration(particle.mass);
                total_acceleration = total_acceleration + acceleration;

                if force.force_type != ForceType::Constant {
                    let idx = self.forces.iter().position(|f| std::ptr::eq(f, force));
                    if let Some(idx) = idx {
                        self.forces.remove(idx);
                    }
                    break;
                }
            }
            accelerations[i] = total_acceleration;
        }

        for (p, acc) in self.particles.iter_mut().zip(accelerations.iter()) {
            p.acceleration = *acc;
            p.velocity = p.velocity + p.acceleration * dt;
            p.position = p.position + p.velocity * dt;
            p.acceleration = Vec2::zero();
        }
    }
}
