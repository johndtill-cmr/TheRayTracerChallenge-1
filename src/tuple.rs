#[derive(Clone)]
#[derive(Copy)]
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

pub fn colour (x: f32, y: f32, z: f32) -> Tuple {
    Tuple {x:x, y:y, z:z, w:0.0}
}

use std::ops;

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, _rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
            w: self.w + _rhs.w,
        }
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, _rhs: Tuple) -> Tuple {
        Tuple {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
            w: self.w - _rhs.w,
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl ops::Mul<f32> for Tuple {
    type Output = Tuple;

    fn mul(self, _rhs: f32) -> Tuple {
        Tuple {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
            w: self.w * _rhs,
        }
    }
}

impl ops::Div<f32> for Tuple {
    type Output = Tuple;

    fn div(self, _rhs: f32) -> Tuple {
        Tuple {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
            w: self.w / _rhs,
        }
    }
}

impl Tuple {
    pub fn is_vector(&self) -> bool { self.w == 0.0 }
    pub fn is_point(&self) -> bool { self.w == 1.0 }
    pub fn is_equal(&self, _rhs: Self) -> bool {
        (self.x == _rhs.x) && (self.y == _rhs.y) && (self.z == _rhs.z) && (self.w == _rhs.w)
    }
    pub fn norm_squared(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w
    }
    pub fn norm(&self) -> f32 {
        f32::sqrt(self.norm_squared())
    }
    pub fn normalised(self) -> Tuple {
        let norm = self.norm();
        self / norm
    }
    pub fn dot(&self, _rhs: Self) -> f32 {
        (self.x * _rhs.x) + (self.y * _rhs.y) + (self.z * _rhs.z) + (self.w * _rhs.w)
    }
    pub fn cross(&self, _rhs: &Self) -> Tuple {
        vector(
            self.y*_rhs.z - self.z*_rhs.y,
            self.z*_rhs.x - self.x*_rhs.z,
            self.x*_rhs.y - self.y*_rhs.x)
    }
    pub fn hadamard_prod(&self, _rhs: &Self) -> Tuple {
        vector(
            self.x*_rhs.x,
            self.y*_rhs.y,
            self.z*_rhs.z)
    }
    pub fn almost_equal(self, _rhs: Self) -> bool {
        let diff = self - _rhs;
        diff.norm_squared() < 1e-12
    }

    pub fn red(&self) -> f32 { self.x }
    pub fn green(&self) -> f32 { self.y }
    pub fn blue(&self) -> f32 { self.z }

    pub fn scale255(x: f32) -> u8 {
        (x*255.0 + 0.5).min(255.0) as u8
    }

    pub fn red255(&self) -> u8 { Tuple::scale255(self.red()) }
    pub fn green255(&self) -> u8 { Tuple::scale255(self.green()) }
    pub fn blue255(&self) -> u8 { Tuple::scale255(self.blue()) }
}