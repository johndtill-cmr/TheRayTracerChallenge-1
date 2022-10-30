pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub fn vector (x: f32, y: f32, z: f32) -> Tuple {
    Tuple {x:x, y:y, z:z, w:0.0}
}

pub fn point (x: f32, y: f32, z: f32) -> Tuple {
    Tuple {x:x, y:y, z:z, w:1.0}
}

impl Tuple {
    pub fn is_vector(&self) -> bool { self.w == 0.0 }
    pub fn is_point(&self) -> bool { self.w == 1.0 }
}