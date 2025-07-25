use std::collections::HashSet;

use crate::{
    force::Force2D,
    traits::Object2D,
};

pub trait Simulator {
    fn update(&mut self, dt: f32) -> Option<&Vec<Box<dyn Object2D>>>;
}

pub struct Simulator2D {
    pub objects: Vec<Box<dyn Object2D>>,
}

impl Simulator2D {
    pub fn new(objects: Vec<Box<dyn Object2D>>) -> Self {
        Self { objects }
    }
}

impl Simulator for Simulator2D {
    fn update(&mut self, dt: f32) -> Option<&Vec<Box<dyn Object2D>>> {
        if self.objects.is_empty() {
            return None;
        }

        for object in &mut self.objects {
            object.update(dt);
        }

        Some(&self.objects)
    }
}
