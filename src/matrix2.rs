pub mod matrix2 {
    use num::Num;
    pub use crate::matrix23_trait::matrix23::Matrix23;
    pub use crate::matrix::matrix::Matrix;

    #[derive(Debug, Default)]
    pub struct Matrix2<T: Num + Default + Copy> {
        rows: usize,
        columns: usize,
        elems: Vec<Vec<T>>,
    }

    impl<T: Num + Default + Copy> Matrix2<T> {
        pub fn new(m11: T, m12: T, m21: T, m22: T) -> Matrix2<T> {
            let e = vec![vec![m11, m12], vec![m21, m22]];
            Matrix2 { rows: 2, columns: 2, elems: e }
        }
    }

    impl<T: Num + Default + Copy> Matrix23<T> for Matrix2<T> {
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

    impl<T: Num + Default + Copy> Matrix<T> for Matrix2<T> {
        fn resize(&mut self) -> &mut Self {
            if self.elems.len() != self.rows {
                if self.elems.first().unwrap().len() != self.columns {
                    let t = self.columns;
                    self.columns = self.rows;
                    self.rows = t;
                }
            }

            self
        }
        fn get_columns(&self) -> usize {
            self.columns
        }

        fn get_rows(&self) -> usize {
            self.rows
        }

        fn get_elements(&mut self) -> &mut Vec<Vec<T>> {
            self.elems.as_mut()
        }
    }
}