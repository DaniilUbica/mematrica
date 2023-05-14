pub mod matrix3 {

    use num::Num;
    pub use crate::matrix23_trait::matrix23::Matrix23;
    pub use crate::matrix::matrix::Matrix;
    use crate::cmatrix::cmatrix::CMatrix;
    use std::ops::Add;
    use core::ops::{Sub, Mul};

    #[derive(Debug, Default)]
    pub struct Matrix3<T: Num + Default + Copy> {
        rows: usize,
        columns: usize,
        elems: Vec<Vec<T>>,
    }

    impl<T: Num + Default + Copy> Matrix3<T> {
        pub fn new(m11: T, m12: T, m13: T, m21: T, m22: T, m23: T, m31: T, m32: T, m33: T,) -> Matrix3<T> {
            let e = vec![vec![m11, m12, m13], vec![m21, m22, m23], vec![m31, m32, m33]];
            Matrix3 { rows: 3, columns: 3, elems: e }
        }
    }

    impl<T: Num + Default + Copy> Matrix23<T> for Matrix3<T> {
        fn zero() -> Self {
            let e = vec![vec![T::zero(), T::zero(), T::zero()], 
            vec![T::zero(), T::zero(), T::zero()], 
            vec![T::zero(), T::zero(), T::zero()]
            ];
            Matrix3 { rows: 3, columns: 3, elems: e }
        }

        fn one() -> Self {
            let e = vec![vec![T::one(), T::one(), T::one()], 
            vec![T::one(), T::one(), T::one()], 
            vec![T::one(), T::one(), T::one()]
            ];
            Matrix3 { rows: 3, columns: 3, elems: e }
        }

        fn from_element(e: T) -> Self {
            let e = vec![vec![e, e, e], 
            vec![e, e, e], 
            vec![e, e, e]
            ];
            Matrix3 { rows: 3, columns: 3, elems: e }
        }

        fn from_vec_as_columns(v: Vec<T>) -> Self {
            let mut t = v.clone();

            if v.len() > 3 {
                panic!("Too much elements in vector!");
            }

            while t.len() < 3 {
                t.push(T::zero());
            }
            
            let e = vec![vec![t[0], t[0], t[0]], 
            vec![t[1], t[1], t[1]],
            vec![t[2], t[2], t[2]],
            ];
            Matrix3 { rows: 3, columns: 3, elems: e }
        }

        fn from_vec_as_rows(v: Vec<T>) -> Self {
            let mut t = v.clone();

            if v.len() > 3 {
                panic!("Too much elements in vector!");
            }

            while t.len() < 3 {
                t.push(T::zero());
            }

            let e = vec![vec![t[0], t[1], t[2]], 
            vec![t[0], t[1], t[2]],
            vec![t[0], t[1], t[2]],
            ];
            Matrix3 { rows: 3, columns: 3, elems: e }
        }
    }

    impl<T: Num + Default + Copy> Matrix<T> for Matrix3<T> {
        fn resize(&mut self) -> &mut Self {
            self.rows = self.elems.len();
            self.columns = self.elems.first().unwrap().len();

            self
        }

        fn get_columns(&self) -> usize {
            self.columns
        }

        fn get_rows(&self) -> usize {
            self.rows
        }

        fn get_elements(&self) -> Vec<Vec<T>> {
            self.elems.clone()
        }

        fn set_elements(&mut self, v: Vec<Vec<T>>) -> &mut Self {
            if self.columns == v.len() && self.columns == v.first().unwrap().len() {
                self.elems = v;
            }
            else {
                panic!("Can't make Matrix3 from this elements! Wrong size maybe?");
            }
            self
        }
    }

    impl<T: Add<Output = T> + Num + Default + Clone + Copy> Add<Matrix3<T>> for Matrix3<T> {
        type Output = Matrix3<T>;

        fn add(self, rhs: Matrix3<T>) -> Matrix3<T> {
            let mut v = self.elems.clone();
            let r = rhs.elems;

            for i in 0..self.rows {
                for j in 0..self.columns {
                    v[i][j] = v[i][j] + r[i][j];
                }
            }

            Matrix3 { rows: self.rows, columns: self.columns, elems: v }
        }
    }

    impl<T: Add<Output = T> + Num + Default + Clone + Copy> Add<CMatrix<T>> for Matrix3<T> {
        type Output = Matrix3<T>;

        fn add(self, rhs: CMatrix<T>) -> Matrix3<T> {
            let mut v = self.elems.clone();

            if self.columns == rhs.get_columns() && self.rows == rhs.get_rows() {
                let r = rhs.get_elements();

                for i in 0..self.rows {
                    for j in 0..self.columns {
                        v[i][j] = v[i][j] + r[i][j];
                    }
                }
            }
            else {
                panic!("Can't fold this matrices: self.columns != rhs.columns || self.elems != rhs.elems");
            }

            Matrix3 { rows: self.rows, columns: self.columns, elems: v }
        }
    }

    impl<T: Sub<Output = T> + Num + Default + Clone + Copy> Sub<Matrix3<T>> for Matrix3<T> {
        type Output = Matrix3<T>;

        fn sub(self, rhs: Matrix3<T>) -> Matrix3<T> {
            let mut v = self.elems.clone();
            let r = rhs.elems;

            for i in 0..self.rows {
                for j in 0..self.columns {
                    v[i][j] = v[i][j] - r[i][j];
                }
            }

            Matrix3 { rows: self.rows, columns: self.columns, elems: v }
        }
    }

    impl<T: Sub<Output = T> + Num + Default + Clone + Copy> Sub<CMatrix<T>> for Matrix3<T> {
        type Output = Matrix3<T>;

        fn sub(self, rhs: CMatrix<T>) -> Matrix3<T> {
            let mut v = self.elems.clone();

            if self.columns == rhs.get_columns() && self.rows == rhs.get_rows() {
                let r = rhs.get_elements();

                for i in 0..self.rows {
                    for j in 0..self.columns {
                        v[i][j] = v[i][j] - r[i][j];
                    }
                }
            }
            else {
                panic!("Can't fold this matrices: self.columns != rhs.columns || self.elems != rhs.elems");
            }

            Matrix3 { rows: self.rows, columns: self.columns, elems: v }
        }
    }

    impl<T: Mul<Output = T> + Num + Default + Clone + Copy> Mul<Matrix3<T>> for Matrix3<T> {
        type Output = Matrix3<T>;

        fn mul(self, rhs: Matrix3<T>) -> Matrix3<T> {
            let mut m = self;
            m.multiplicate(rhs);
            Matrix3 { rows: 3, columns: 3, elems: m.elems }
        }
    }

    impl<T: Mul<Output = T> + Num + Default + Clone + Copy> Mul<CMatrix<T>> for Matrix3<T> {
        type Output = CMatrix<T>;

        fn mul(self, rhs: CMatrix<T>) -> CMatrix<T> {
            let mut m = self;
            m.multiplicate(rhs)
        }
    }
}