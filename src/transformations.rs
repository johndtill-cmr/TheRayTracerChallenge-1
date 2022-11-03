use crate::matrix::*;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
    Matrix::new(4, 4, [1.0, 0.0, 0.0, x, 0.0, 1.0, 0.0, y, 0.0, 0.0, 1.0, z, 0.0, 0.0, 0.0, 1.0].to_vec())
}

pub fn scaling(x: f32, y: f32, z: f32) -> Matrix {
    Matrix::new(4, 4, [x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0].to_vec())
}

pub fn rotation_x(angle: f32) -> Matrix {
    let c = angle.cos();
    let s = angle.sin();
    Matrix::new(4, 4, [1.0, 0.0, 0.0, 0.0,
                       0.0, c, -s, 0.0,
                       0.0, s, c, 0.0,
                       0.0, 0.0, 0.0, 1.0].to_vec())
}

pub fn rotation_y(angle: f32) -> Matrix {
    let c = angle.cos();
    let s = angle.sin();
    Matrix::new(4, 4, [c, 0.0, s, 0.0,
                       0.0, 1.0, 0.0, 0.0,
                       -s, 0.0, c, 0.0,
                       0.0, 0.0, 0.0, 1.0].to_vec())
}

pub fn rotation_z(angle: f32) -> Matrix {
    let c = angle.cos();
    let s = angle.sin();
    Matrix::new(4, 4, [c, -s, 0.0, 0.0,
                       s, c, 0.0, 0.0,
                       0.0, 0.0, 1.0, 0.0,
                       0.0, 0.0, 0.0, 1.0].to_vec())
}

pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix {
    Matrix::new(4, 4, [1.0, xy, xz, 0.0, yx, 1.0, yz, 0.0, zx, zy, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0].to_vec())
}