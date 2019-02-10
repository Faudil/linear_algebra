#[cfg(test)]
extern crate neural_network;

mod matrix_tests {
    use neural_network::math::matrix::Matrix;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn simple_addition() {
        let mut m = Matrix::new_init(2, 2, vec![1.0, 1.0, 2.0, 2.0]);
        let n = Matrix::new_init(2, 2, vec![1.0, 1.0, 2.0, 2.0]);
        let r = Matrix::new_init(2, 2, vec![2.0, 2.0, 4.0, 4.0]);
        m.add(&n);
        assert_eq!(m.eq(&r), true);
    }

    #[test]
    fn simple_substraction() {
        let mut m = Matrix::new_init(2, 2, vec![1.0, 1.0, 2.0, 2.0]);
        let n = Matrix::new_init(2, 2, vec![1.0, 1.0, 2.0, 2.0]);
        let r = Matrix::new_init(2, 2, vec![0.0, 0.0, 0.0, 0.0]);
        m.sub(&n);
        assert_eq!(m.eq(&r), true);
    }

    #[test]
    fn simple_multiplication() {
        let mut m = Matrix::new_init(2, 2, vec![1.0, 1.0, 0.0, 1.0]);
        let n = Matrix::new_init(2, 2, vec![2.0, 1.0, 1.0, 1.0]);
        let r = Matrix::new_init(2, 2, vec![3.0, 2.0, 1.0, 1.0]);
        let t = m * n;
        assert_eq!(t.eq(&r), true);
    }
}