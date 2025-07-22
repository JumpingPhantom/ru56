use crate::simulation::{Simulator};

pub trait Renderer {}

pub struct Renderer2D {}

impl Renderer2D {
    fn new() -> Self {
        Renderer2D {}
    }
}

impl Renderer for Renderer2D {}

pub struct Controller<'a, S: Simulator, R: Renderer> {
    simulator: Option<&'a mut S>,
    renderer: Option<R>,
}

impl<'a, S: Simulator, R: Renderer> Controller<'a, S, R> {
    pub fn new() -> Self {
        Controller {
            simulator: None,
            renderer: None,
        }
    }

    pub fn simulator(mut self, simulator: &'a mut S) -> Self {
        self.simulator = Some(simulator);
        self
    }

    pub fn renderer(mut self, renderer: R) -> Self {
        self.renderer = Some(renderer);
        self
    }

    pub fn execute(&mut self) {
        if let Some(sim) = self.simulator.as_mut() {
            let mut last_time = std::time::Instant::now();

            loop {
                let now = std::time::Instant::now();
                let dt = now.duration_since(last_time).as_secs_f32();
                last_time = now;

                sim.update(dt);
            }
        }
    }
}