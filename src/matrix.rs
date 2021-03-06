use std::fmt;
use std::ops::{Index, Mul};
use std::cmp::{PartialEq};

pub struct Matrix {
    _height: usize,
    _width: usize,
    _bound: usize,
    _value: Vec<f64>
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_list = self._value.iter().map(|&x| x.to_string()).collect::<Vec<String>>();
        write!(f, "{}", format!("Matrix :: height = {} width = {}\n value = [{}]", self._height, self._width, str_list.join(", ")))
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_list = self._value.iter().map(|&x| x.to_string()).collect::<Vec<String>>();
        write!(f, "{}", format!("h = {} w = {} value = [{}]", self._height, self._width, str_list.join(", ")))
    }
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
            if (self._value[i] - other._value[i]).abs() > 0.0001 {
                return false;
            }
        }
        true
    }
}

impl Mul<f64> for Matrix {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        let mut offset;
        let mut res = self.clone();
        for i in 0..self._height {
            offset = i * res._width;
            for j in 0..self._width {
                res._value[offset + j] *= other;
            }
        }
        res
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut offset;
        let mut res = Matrix::new(self._height, other._width);
        for i in 0..self._height {
            offset = i * res._width;
            for j in 0..self._width {
                res._value[offset + j] = self.mul_line(&other, i, j);
            }
        }
        res
    }
}


impl Matrix {
    pub fn new(height: usize, width: usize) -> Matrix {
        Matrix {_height: height, _width: width, _bound: height * width, _value: vec![0.0; height * width]}
    }

    pub fn new_init(height: usize, width: usize, data: Vec<f64>) -> Matrix {
        Matrix {_height: height, _width: width, _bound: height * width, _value: data}
    }

    pub fn access(&self, x: usize, y: usize) -> f64 {
        self._value[y * self._width + x]
    }

    fn mul_line(&self, other: &Matrix, i: usize, j: usize) -> f64{
        let mut res = 0.0;
            for k in 0..other._width {
                res += self._value[i * self._width + k] * other._value[k * self._width + j]
            }
        res
    }

    pub fn add(&mut self, other: &Matrix) {
        for i in 0..self._bound {
            self._value[i] += other._value[i];
        }
    }

    pub fn sub(&mut self, other: &Matrix) {
        for i in 0..self._bound {
            self._value[i] -= other._value[i];
        }
    }

    pub fn transpose(&self) -> Matrix {
        let mut new_matrix = Matrix::new(self._width, self._height);
        for x in 0..self._width {
            for y in 0..self._height {
                new_matrix._value[x * self._height + y] = self._value[y * self._height + x];
            }
        }
        new_matrix
    }

    pub fn clone(&self) -> Matrix {
        let a = self._value.clone();
        Matrix::new_init(self._height, self._width, a)
    }

    pub fn map(&self, fun: (fn (f64) -> f64)) -> Matrix {
        let mut new_matrix = self.clone();
        new_matrix.apply_map(fun);
        return new_matrix;
    }

    pub fn apply_map(& mut self, fun: (fn (f64) -> f64)) {
        let len = self._value.len();
        for i in 0..len {
            self._value[i] = fun(self._value[i]);
        }
    }
}