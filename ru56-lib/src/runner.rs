use crate::force::{Force2D, ForceType};
use crate::particle::Particle2D;
use crate::vector::Vec2;

pub struct Simulation2D {
    pub particles: Vec<Particle2D>,
    pub forces: Vec<Force2D>,
    pub dt: f32,
}

impl Simulation2D {
    pub fn new(particles: Vec<Particle2D>, forces: Vec<Force2D>, dt: f32) -> Self {
        Self {
            particles,
            forces,
            dt,
        }
    }

    pub fn update(&mut self) {
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
            p.velocity = p.velocity + p.acceleration * self.dt;
            p.position = p.position + p.velocity * self.dt;
            p.acceleration = Vec2::zero();
        }
    }
}

pub struct Runner2D<'a> {
    pub sim: &'a mut Simulation2D,
    pub steps: usize,
}

impl<'a> Runner2D<'a> {
    pub fn new(sim: &'a mut Simulation2D, steps: usize) -> Self {
        Self { sim, steps }
    }

    pub fn run(&mut self) {
        let mut last_time = std::time::Instant::now();

        for _ in 0..self.steps {
            let now = std::time::Instant::now();
            let dt = now.duration_since(last_time).as_secs_f32();
            last_time = now;

            self.sim.dt = dt;
            self.sim.update();

            for p in self.sim.particles.iter() {
                println!("vx: {}, posx: {}", p.velocity.x, p.position.x);
            }
        }
    }
}
