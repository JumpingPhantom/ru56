use crate::{simulation::Simulator, traits::Renderer};


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
        let mut last_time = std::time::Instant::now();

        match (&mut self.simulator, &mut self.renderer) {
            (Some(sim), ren_opt) => loop {
                let now = std::time::Instant::now();
                let dt = now.duration_since(last_time).as_secs_f32();
                last_time = now;
                let data = sim.update(dt);
                if let Some(ren) = ren_opt {
                    if let Some(data) = data {
                        ren.render(data);
                    }
                }
            },
            _ => {}
        }
    }
}
