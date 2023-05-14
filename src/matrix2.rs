pub mod matrix2 {
    use num::Num;
    pub use crate::matrix23_trait::matrix23::Matrix23;
    pub use crate::matrix::matrix::Matrix;
    use crate::cmatrix::cmatrix::CMatrix;
    use std::ops::Add;
    use core::ops::{Sub, Mul};

    #[derive(Debug, Default)]
    pub struct Matrix2<T: Num + Default + Copy + PartialOrd> {
        rows: usize,
        columns: usize,
        elems: Vec<Vec<T>>,
    }

    impl<T: Num + Default + Copy + PartialOrd> Matrix2<T> {
        pub fn new(m11: T, m12: T, m21: T, m22: T) -> Matrix2<T> {
            let e = vec![vec![m11, m12], vec![m21, m22]];
            Matrix2 { rows: 2, columns: 2, elems: e }
        }
    }

    impl<T: Num + Default + Copy + PartialOrd> Matrix23<T> for Matrix2<T> {
        fn zero() -> Self {
            let e = vec![vec![T::zero(), T::zero()], vec![T::zero(), T::zero()]];
            Matrix2 { rows: 2, columns: 2, elems: e }
        }

        fn one() -> Self {
            let e = vec![vec![T::one(), T::one()], vec![T::one(), T::one()]];
            Matrix2 { rows: 2, columns: 2, elems: e }
        }

        fn from_element(e: T) -> Self {
            let e = vec![vec![e, e], vec![e, e]];
            Matrix2 { rows: 2, columns: 2, elems: e }
        }

        fn from_vec_as_columns(v: Vec<T>) -> Self {
            let mut t = v.clone();

            if v.len() > 2 {
                panic!("Too much elements in vector!");
            }

            while t.len() < 2 {
                t.push(T::zero());
            }

            let e = vec![vec![t[0], t[0]], vec![t[1], t[1]]];
            Matrix2 { rows: 2, columns: 2, elems: e }
        }

        fn from_vec_as_rows(v: Vec<T>) -> Self {
            let mut t = v.clone();

            if v.len() > 2 {
                panic!("Too much elements in vector!");
            }

            while t.len() < 2 {
                t.push(T::zero());
            }

            let e = vec![vec![t[0], t[1]], vec![t[0], t[1]]];
            Matrix2 { rows: 2, columns: 2, elems: e }
        }
    }

    impl<T: Num + Default + Copy + PartialOrd> Matrix<T> for Matrix2<T> {
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
                panic!("Can't make Matrix2 from this elements! Wrong size maybe?");
            }
            self
        }
    }

    impl<T: Add<Output = T> + Num + Default + Clone + Copy + PartialOrd> Add<Matrix2<T>> for Matrix2<T> {
        type Output = Matrix2<T>;

        fn add(self, rhs: Matrix2<T>) -> Matrix2<T> {
            let mut v = self.elems.clone();
            let r = rhs.elems;

            for i in 0..self.rows {
                for j in 0..self.columns {
                    v[i][j] = v[i][j] + r[i][j];
                }
            }

            Matrix2 { rows: self.rows, columns: self.columns, elems: v }
        }
    }

    impl<T: Add<Output = T> + Num + Default + Clone + Copy + PartialOrd> Add<CMatrix<T>> for Matrix2<T> {
        type Output = Matrix2<T>;

        fn add(self, rhs: CMatrix<T>) -> Matrix2<T> {
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

            Matrix2 { rows: self.rows, columns: self.columns, elems: v }
        }
    }

    impl<T: Sub<Output = T> + Num + Default + Clone + Copy + PartialOrd> Sub<Matrix2<T>> for Matrix2<T> {
        type Output = Matrix2<T>;

        fn sub(self, rhs: Matrix2<T>) -> Matrix2<T> {
            let mut v = self.elems.clone();
            let r = rhs.elems;

            for i in 0..self.rows {
                for j in 0..self.columns {
                    v[i][j] = v[i][j] - r[i][j];
                }
            }

            Matrix2 { rows: self.rows, columns: self.columns, elems: v }
        }
    }

    impl<T: Sub<Output = T> + Num + Default + Clone + Copy + PartialOrd> Sub<CMatrix<T>> for Matrix2<T> {
        type Output = Matrix2<T>;

        fn sub(self, rhs: CMatrix<T>) -> Matrix2<T> {
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

            Matrix2 { rows: self.rows, columns: self.columns, elems: v }
        }
    }

    impl<T: Mul<Output = T> + Num + Default + Clone + Copy + PartialOrd> Mul<Matrix2<T>> for Matrix2<T> {
        type Output = Matrix2<T>;

        fn mul(self, rhs: Matrix2<T>) -> Matrix2<T> {
            let mut m = self;
            m.multiplicate(rhs);
            Matrix2 { rows: 2, columns: 2, elems: m.elems }
        }
    }

    impl<T: Mul<Output = T> + Num + Default + Clone + Copy + PartialOrd> Mul<CMatrix<T>> for Matrix2<T> {
        type Output = CMatrix<T>;

        fn mul(self, rhs: CMatrix<T>) -> CMatrix<T> {
            let mut m = self;
            m.multiplicate(rhs)
        }
    }
}