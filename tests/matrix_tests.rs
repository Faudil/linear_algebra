#[cfg(test)]
extern crate linear_algebra;

mod matrix_tests {
    use linear_algebra::matrix::Matrix;
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
        assert_eq!(m, r);
    }

    #[test]
    fn simple_substraction() {
        let mut m = Matrix::new_init(2, 2, vec![1.0, 1.0, 2.0, 2.0]);
        let n = Matrix::new_init(2, 2, vec![1.0, 1.0, 2.0, 2.0]);
        let r = Matrix::new_init(2, 2, vec![0.0, 0.0, 0.0, 0.0]);
        m.sub(&n);
        assert_eq!(m, r);
    }

    #[test]
    fn simple_multiplication() {
        let m = Matrix::new_init(2, 2, vec![1.0, 1.0,
                                                                      0.0, 1.0]);
        let n = Matrix::new_init(2, 2, vec![2.0, 1.0,
                                                                      1.0, 1.0]);
        let r = Matrix::new_init(2, 2, vec![3.0, 2.0,
                                                                      1.0, 1.0]);
        let t = m * n;
        assert_eq!(t, r);
    }

    #[test]
    fn simple_clone() {
        let a = Matrix::new_init(2, 2, vec![2.0, 3.0, 5.0, 7.25]);
        assert_eq!(a, a.clone());
    }
}