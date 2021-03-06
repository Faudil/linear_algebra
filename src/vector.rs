
/*impl Index<f64> for Vector {
    type Output = ();

    fn index(&self, index: f64) -> &f64 {
        &self._value[index]
    }
}*/

pub struct Vector {
    _size: usize,
    _value: Vec<f64>
}


impl Vector {
    pub fn new(size: usize) -> Vector {
        Vector {_size: size, _value: vec![0.0; size]}
    }
}