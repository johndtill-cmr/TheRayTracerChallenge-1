#[derive(Clone)]
pub struct Matrix {
    pub width: u16,
    pub height: u16,
    pub data: Vec<f32>,
}

use std::ops;

impl std::ops::Index<[usize; 2]> for Matrix {
    type Output = f32;

    fn index(&self, idx: [usize; 2]) -> &f32 {
        assert!(idx[1] < (self.width as usize));
        &self.data[idx[0] * (self.width as usize) + idx[1]]
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, _rhs: Matrix) -> Matrix {
        assert_eq!(self.width, _rhs.height);

        let mut data : Vec<f32> = vec![];
        for i in 0 .. (self.height as usize) {
            for j in 0 .. (_rhs.width as usize) {
                let mut elem = 0.0;
                for k in 0 .. (self.width as usize) {
                    elem += self[[i, k]] * _rhs[[k, j]];
                }

                data.push(elem);
            }
        }

        Matrix {
            width: _rhs.width,
            height: self.height,
            data: data,
        }
    }
}

use crate::tuple::*;

impl ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, _rhs: Tuple) -> Tuple {
        assert_eq!(self.height, 4);
        assert_eq!(self.width, 4);

        Tuple {
            x: self[[0,0]]*_rhs.x + self[[0,1]]*_rhs.y + self[[0,2]]*_rhs.z + self[[0,3]]*_rhs.w,
            y: self[[1,0]]*_rhs.x + self[[1,1]]*_rhs.y + self[[1,2]]*_rhs.z + self[[1,3]]*_rhs.w,
            z: self[[2,0]]*_rhs.x + self[[2,1]]*_rhs.y + self[[2,2]]*_rhs.z + self[[2,3]]*_rhs.w,
            w: self[[3,0]]*_rhs.x + self[[3,1]]*_rhs.y + self[[3,2]]*_rhs.z + self[[3,3]]*_rhs.w,
        }
    }
}

impl Matrix {
    pub fn zero(width: u16, height: u16) -> Self {
        Matrix {
            width: width,
            height: height,
            data: vec![0.0; (width as usize)*(height as usize)],
        }
    }

    pub fn identity(width: u16, height: u16) -> Self {
        let mut m = Matrix {
            width: width,
            height: height,
            data: vec![0.0; (width as usize)*(height as usize)],
        };

        for i in (0 .. (width as usize)*(height as usize)).step_by((width as usize)+1) {
            m.data[i] = 1.0;
        }

        m
    }

    pub fn new(width: u16, height: u16, data: Vec<f32>) -> Self {
        assert_eq!((width as usize)*(height as usize), data.len());

        Matrix {
            width: width,
            height: height,
            data: data,
        }
    }

    pub fn almost_equal(self, _rhs: Self) -> bool {
        if self.width != _rhs.width || self.height != _rhs.height { return false; }
        for i in 0 .. self.data.len() {
            let diff = self.data[i] - _rhs.data[i];
            if diff > 1e-4 || diff < -1e-4 { return false; }
        }
        true
    }

    pub fn transpose(&self) -> Matrix {
        let mut data : Vec<f32> = vec![];

        for j in 0 .. (self.width as usize) {
            for i in 0 .. (self.height as usize) {
                data.push(self[[i, j]]);
            }
        }

        Matrix {
            width: self.height,
            height: self.width,
            data: data,
        }
    }

    pub fn submatrix(&self, row: u16, col: u16) -> Matrix {
        let mut data : Vec<f32> = vec![];

        for i in 0 .. self.height {
            for j in 0 .. self.width {
                if i != row && j != col {
                    data.push(self[[i as usize, j as usize]]);
                }
            }
        }

        Matrix {
            width: self.height-1,
            height: self.width-1,
            data: data,
        }
    }

    pub fn det2x2(&self) -> f32 {
        assert_eq!(self.width, 2);
        assert_eq!(self.height, 2);

        self[[0, 0]]*self[[1, 1]] - self[[0, 1]]*self[[1, 0]]
    }

    pub fn minor(&self, row: u16, col: u16) -> f32 {
        assert_eq!(self.width, self.height);
        self.submatrix(row, col).det()
    }

    pub fn cofactor(&self, row: u16, col: u16) -> f32 {
        self.minor(row, col) * if (row+col) % 2 == 0 {1.0} else {-1.0}
    }

    pub fn det(&self) -> f32 {
        assert_eq!(self.width, self.height);
        assert!(self.width >= 2);

        if self.width == 2 { return self.det2x2(); }

        let mut det : f32 = 0.0;
        for i in 0 .. self.width {
            det += self.data[i as usize] * self.cofactor(0, i);
        }

        return det;
    }

    pub fn is_invertible(&self) -> bool {
        let d = self.det();
        d > 1e-6 || d < -1e-6
    }

    pub fn inverse(&self) -> Matrix {
        assert!(self.is_invertible());

        let det = self.det();
        
        let mut data : Vec<f32> = vec![];
        for i in 0 .. self.height {
            for j in 0 .. self.width {
                data.push(self.cofactor(j, i) / det);
            }
        }

        Matrix {
            width: self.width,
            height: self.height,
            data: data,
        }
    }
}