use crate::tuple::*;
use crate::spheres::*;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn position(&self, distance: f32) -> Tuple {
        self.origin + self.direction * distance
    }
	
	pub fn intersect(&self, sphere: Sphere) -> Vec<f32> {
		let mut intersecting_distances : Vec<f32> = vec![];
		return intersecting_distances; //DO THIS
	}
}