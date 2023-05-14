pub mod cmatrix {
    use num::Num;
    pub use crate::cmatrix_trait::cmatrix_trait::CMatrixTrait;
    pub use crate::matrix::matrix::Matrix;
    use crate::matrix2::matrix2::Matrix2;
    use crate::matrix3::matrix3::Matrix3;
    use std::ops::Add;
    use core::ops::{Sub, Mul};

    #[derive(Debug, Default)]
    pub struct CMatrix<T: Num + Default + Clone + PartialOrd> {
        rows: usize,
        columns: usize,
        elems: Vec<Vec<T>>,
    }

    impl<T: Num + Default + Clone + PartialOrd> CMatrixTrait<T> for CMatrix<T> {
        fn zero(rows: usize, columns: usize) -> Self {
            let mut e = vec![];
            let mut v = vec![];

            for _i in 0..columns {
                v.push(T::zero());
            }

            for _i in 0..rows {
                e.push(v.clone());
            }

            CMatrix { rows, columns, elems: e }
        }

        fn one(rows: usize, columns: usize) -> Self {
            let mut e = vec![];
            let mut v = vec![];

            for _i in 0..columns {
                v.push(T::one());
            }

            for _i in 0..rows {
                e.push(v.clone());
            }

            CMatrix { rows, columns, elems: e }
        }

        fn from_element(rows: usize, columns: usize, el: T) -> Self {
            let mut e = vec![];
            let mut v = vec![];

            for _i in 0..columns {
                v.push(el.clone());
            }

            for _i in 0..rows {
                e.push(v.clone());
            }

            CMatrix { rows, columns, elems: e }
        }

        fn from_vec_as_columns(columns: usize, v: Vec<T>) -> Self {
            let mut e = vec![];
            let mut t = vec![];
            let rows = v.len();

            for i in 0..rows {
                for _j in 0..columns {
                    t.push(v[i].clone());
                }
                e.push(t.clone());
                t.clear();
            }

            CMatrix { rows, columns, elems: e }
        }

        fn from_vec_as_rows(rows: usize, v: Vec<T>) -> Self {
            let mut e = vec![];

            for _i in 0..rows {
                e.push(v.clone());
            }

            let columns = v.len();

            CMatrix { rows, columns , elems: e }
        }
    }
    
    impl<T: Num + Default + Clone + PartialOrd> Matrix<T> for CMatrix<T> {
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
            self.rows = v.len();
            self.columns = v[0].len();
            self.elems = v;

            self.resize()
        }
    }

    impl<T: Add<Output = T> + Num + Default + Clone + Copy + PartialOrd> Add<Matrix3<T>> for CMatrix<T> {
        type Output = CMatrix<T>;

        fn add(self, rhs: Matrix3<T>) -> CMatrix<T> {
            let mut v = self.elems.clone();
            let r = rhs.get_elements();

            if self.columns == rhs.get_columns() && self.rows == rhs.get_rows() {
                for i in 0..self.rows {
                    for j in 0..self.columns {
                        v[i][j] = v[i][j] + r[i][j];
                    }
                }
            }
            else {
                panic!("Can't fold this matrices: self.columns != rhs.columns || self.rows != rhs.rows");
            }

            CMatrix { rows: self.rows, columns: self.columns, elems: v }
        }
    }

    impl<T: Add<Output = T> + Num + Default + Clone + Copy + PartialOrd> Add<Matrix2<T>> for CMatrix<T> {
        type Output = CMatrix<T>;

        fn add(self, rhs: Matrix2<T>) -> CMatrix<T> {
            let mut v = self.elems.clone();
            let r = rhs.get_elements();

            if self.columns == rhs.get_columns() && self.rows == rhs.get_rows() {
                for i in 0..self.rows {
                    for j in 0..self.columns {
                        v[i][j] = v[i][j] + r[i][j];
                    }
                }
            }
            else {
                panic!("Can't fold this matrices: self.columns != rhs.columns || self.rows != rhs.rows");
            }
            
            CMatrix { rows: self.rows, columns: self.columns, elems: v }
        }
    }

    impl<T: Add<Output = T> + Num + Default + Clone + Copy + PartialOrd> Add<CMatrix<T>> for CMatrix<T> {
        type Output = CMatrix<T>;

        fn add(self, rhs: CMatrix<T>) -> CMatrix<T> {
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
                panic!("Can't fold this matrices: self.columns != rhs.columns || self.rows != rhs.rows");
            }

            CMatrix { rows: self.rows, columns: self.columns, elems: v }
        }
    }

    impl<T: Sub<Output = T> + Num + Default + Clone + Copy + PartialOrd> Sub<Matrix3<T>> for CMatrix<T> {
        type Output = CMatrix<T>;

        fn sub(self, rhs: Matrix3<T>) -> CMatrix<T> {
            let mut v = self.elems.clone();
            let r = rhs.get_elements();

            if self.columns == rhs.get_columns() && self.rows == rhs.get_rows() {
                for i in 0..self.rows {
                    for j in 0..self.columns {
                        v[i][j] = v[i][j] - r[i][j];
                    }
                }
            }
            else {
                panic!("Can't fold this matrices: self.columns != rhs.columns || self.rows != rhs.rows");
            }

            CMatrix { rows: self.rows, columns: self.columns, elems: v }
        }
    }

    impl<T: Sub<Output = T> + Num + Default + Clone + Copy + PartialOrd> Sub<Matrix2<T>> for CMatrix<T> {
        type Output = CMatrix<T>;

        fn sub(self, rhs: Matrix2<T>) -> CMatrix<T> {
            let mut v = self.elems.clone();
            let r = rhs.get_elements();

            if self.columns == rhs.get_columns() && self.rows == rhs.get_rows() {
                for i in 0..self.rows {
                    for j in 0..self.columns {
                        v[i][j] = v[i][j] - r[i][j];
                    }
                }
            }
            else {
                panic!("Can't fold this matrices: self.columns != rhs.columns || self.rows != rhs.rows");
            }
            
            CMatrix { rows: self.rows, columns: self.columns, elems: v }
        }
    }

    impl<T: Sub<Output = T> + Num + Default + Clone + Copy + PartialOrd> Sub<CMatrix<T>> for CMatrix<T> {
        type Output = CMatrix<T>;

        fn sub(self, rhs: CMatrix<T>) -> CMatrix<T> {
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
                panic!("Can't fold this matrices: self.columns != rhs.columns || self.rows != rhs.rows");
            }

            CMatrix { rows: self.rows, columns: self.columns, elems: v }
        }
    }

    impl<T: Mul<Output = T> + Num + Default + Clone + Copy + PartialOrd> Mul<Matrix3<T>> for CMatrix<T> {
        type Output = CMatrix<T>;

        fn mul(self, rhs: Matrix3<T>) -> CMatrix<T> {
            let mut m = self;
            m.multiplicate(rhs)
        }
    }

    impl<T: Mul<Output = T> + Num + Default + Clone + Copy + PartialOrd> Mul<Matrix2<T>> for CMatrix<T> {
        type Output = CMatrix<T>;

        fn mul(self, rhs: Matrix2<T>) -> CMatrix<T> {
            let mut m = self;
            m.multiplicate(rhs)
        }
    }

    impl<T: Mul<Output = T> + Num + Default + Clone + Copy + PartialOrd> Mul<CMatrix<T>> for CMatrix<T> {
        type Output = CMatrix<T>;

        fn mul(self, rhs: CMatrix<T>) -> CMatrix<T> {
            let mut m = self;
            m.multiplicate(rhs)
        }
    }
}