use std::ops::{Add, Mul};

use crate::vector::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ForceType {
    Pulse,
    Constant,
}

pub struct Force2D {
    pub force_type: ForceType,
    pub force_vector: Vec2,
}

impl Add for Force2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            force_type: self.force_type,
            force_vector: self.force_vector + rhs.force_vector,
        }
    }
}

impl Mul<f32> for Force2D {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            force_type: self.force_type,
            force_vector: self.force_vector * rhs,
        }
    }
}

impl Force2D {
    pub fn magnitude(&self) -> f32 {
        self.force_vector.length()
    }

    pub fn direction(&self) -> Vec2 {
        self.force_vector.normalize()
    }

    pub fn acceleration(&self, mass: f32) -> Vec2 {
        if mass == 0.0 {
            Vec2::zero()
        } else {
            self.force_vector / mass
        }
    }
}
