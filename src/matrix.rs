use std::ops::{Index, Mul};
use std::cmp::{Eq, PartialEq};

pub struct Matrix {
    _height: usize,
    _width: usize,
    _value: Vec<f64>
}

impl Index<usize> for Matrix {
    type Output = f64;
    fn index<>(&self, i: usize) -> &f64 {
        &self._value[i]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if self._height * self._width != other._height * other._width {
            return false;
        }
        for i in 0..self._width * self._height {
            if self._value[i] - other._value[i] > 0.0001 {
                return false;
            }
        }
        true
    }
}

impl Eq for Matrix {
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut offset;
        let mut res = Matrix::new(self._height, other._width);
        for i in 0..self._height {
            offset = i * res._height;
            for j in 0..self._width {
                res._value[offset + j] = self.mul_line(&other, i, j);
            }
        }
        res
    }
}

impl Matrix {
    pub fn new(height: usize, width: usize) -> Matrix {
        Matrix {_height: height, _width: width, _value: vec![0.0; height * width]}
    }

    pub fn new_init(height: usize, width: usize, data: Vec<f64>) -> Matrix {
        Matrix {_height: height, _width: width, _value: data}
    }

    pub fn access(&self, x: usize, y: usize) -> f64 {
        self._value[y * self._width + x]
    }

    fn mul_line(&self, other: &Matrix, i: usize, j: usize) -> f64{
        let mut res = 0.0;
        for y in i..self._height - 1 {
            for x in j..other._width - 1 {
                res += self._value[y * self._height + x] * other._value[x * self._height + y]
            }
        }
        res
    }

    pub fn add(&mut self, other: &Matrix) {
        let mut offset;
        let mut index;
        for y in 0..self._height {
            offset = y * self._height;
            for x in 0..self._width {
                index = offset + x;
                self._value[index] += other[index];
            }
        }
    }

    pub fn sub(&mut self, other: &Matrix) {
        let mut offset;
        let mut index;
        for y in 0..self._height {
            offset = y * self._height;
            for x in 0..self._width {
                index = offset + x;
                self._value[index] -= other[index];
            }
        }
    }
}