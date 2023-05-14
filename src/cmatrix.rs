pub mod cmatrix {
    use num::Num;
    pub use crate::cmatrix_trait::cmatrix_trait::CMatrixTrait;
    pub use crate::matrix::matrix::Matrix;

    #[derive(Debug, Default)]
    pub struct CMatrix<T: Num + Default + Copy> {
        rows: usize,
        columns: usize,
        elems: Vec<Vec<T>>,
    }

    impl<T: Num + Default + Copy> CMatrixTrait<T> for CMatrix<T> {
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
                v.push(el);
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
                    t.push(v[i]);
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
    
    impl<T: Num + Default + Copy> Matrix<T> for CMatrix<T> {
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