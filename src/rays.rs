use crate::tuple::*;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn position(&self, distance: f32) -> Tuple {
        self.origin + self.direction * distance
    }
}