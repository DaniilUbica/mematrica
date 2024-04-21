pub mod cmatrix {
    extern crate num;

    pub use crate::cmatrix_trait::cmatrix_trait::CMatrixTrait;
    pub use crate::matrix::matrix::Matrix;
    use crate::matrix2::matrix2::Matrix2;
    use crate::matrix3::matrix3::Matrix3;
    use crate::{Error, Matrix23};

    use self::num::Num;
    use std::{fs::OpenOptions, io::Read};

    #[derive(Debug, Default, Clone, Eq)]
    pub struct CMatrix<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> {
        pub(crate) rows: usize,
        pub(crate) columns: usize,
        pub(crate) elems: Vec<Vec<T>>,
    }

    impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> CMatrixTrait<T> for CMatrix<T> {
        fn zero(rows: usize, columns: usize) -> Self {
            let mut e = vec![];
            let mut v = vec![];

            for _i in 0..columns {
                v.push(T::zero());
            }

            for _i in 0..rows {
                e.push(v.clone());
            }

            CMatrix {
                rows,
                columns,
                elems: e,
            }
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

            CMatrix {
                rows,
                columns,
                elems: e,
            }
        }

        fn identity(rows: usize, columns: usize) -> Self {
            let m = CMatrix::zero(rows, columns);
            let mut m = m.get_elements();

            for i in 0..rows {
                for _ in 0..columns {
                    m[i][i] = T::one();
                }
            }

            CMatrix {
                rows,
                columns,
                elems: m,
            }
        }

        fn from_file(filename: String, delimiter: char, rows: usize, columns: usize) -> Self
        where
            <T as std::str::FromStr>::Err: std::fmt::Debug,
        {
            let mut file = OpenOptions::new()
                .read(true)
                .open(filename.clone())
                .expect(&format!("Can't open file with filename {filename}"));

            let mut s = String::new();

            file.read_to_string(&mut s)
                .expect(&format!("Can't read file with filename {filename}"));

            s = s.trim().to_string();

            let mut e: Vec<&str> = s.split(delimiter).collect();

            if e.len() < rows * columns {
                for _ in e.len()..rows * columns {
                    e.push("0");
                }
            }

            let mut e: Vec<T> = e
                .iter()
                .map(|c| {
                    c.parse()
                        .expect("Can't parse file. Maybe some errors in delimiters?")
                })
                .collect();

            let t = rows * columns;

            for _ in 0..t {
                if e.len() < t {
                    e.push(T::zero());
                }
            }

            let mut v = vec![];
            let mut q = vec![];

            let mut j = 0;
            while j < e.len() {
                for _ in 0..rows {
                    q.push(e[j].clone());
                    j += 1;
                }
                v.push(q.clone());
                q.clear();
            }

            let mut r = CMatrix::one(rows, columns);

            r.set_elements(v);

            r
        }

        fn try_from_file(
            filename: String,
            delimiter: char,
            rows: usize,
            columns: usize,
        ) -> Result<Self, Error>
        where
            <T as std::str::FromStr>::Err: std::fmt::Debug,
        {
            let file = OpenOptions::new().read(true).open(filename.clone());

            let mut file = match file {
                Ok(file) => file,
                Err(_) => return Err(Error(format!("Can't open file with filename '{filename}'"))),
            };

            let mut s = String::new();

            let res = file.read_to_string(&mut s);

            match res {
                Ok(_) => (),
                Err(_) => return Err(Error(format!("Can't read file with filename '{filename}'"))),
            };

            s = s.trim().to_string();

            let mut e: Vec<&str> = s.split(delimiter).collect();

            if e.len() < rows * columns {
                for _ in e.len()..rows * columns {
                    e.push("0");
                }
            }

            let mut e: Vec<T> = e
                .iter()
                .map(|c| {
                    c.parse()
                        .expect("Can't parse file. Maybe some errors in delimiters?")
                })
                .collect();

            let t = rows * columns;

            for _ in 0..t {
                if e.len() < t {
                    e.push(T::zero());
                }
            }

            let mut v = vec![];
            let mut q = vec![];

            let mut j = 0;
            while j < e.len() {
                for _ in 0..rows {
                    q.push(e[j].clone());
                    j += 1;
                }
                v.push(q.clone());
                q.clear();
            }

            let mut r = CMatrix::one(rows, columns);

            r.set_elements(v);

            Ok(r)
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

            CMatrix {
                rows,
                columns,
                elems: e,
            }
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

            CMatrix {
                rows,
                columns,
                elems: e,
            }
        }

        fn from_vec_as_rows(rows: usize, v: Vec<T>) -> Self {
            let mut e = vec![];

            for _i in 0..rows {
                e.push(v.clone());
            }

            let columns = v.len();

            CMatrix {
                rows,
                columns,
                elems: e,
            }
        }

        fn from_diagonal(rows: usize, columns: usize, element: T) -> Self {
            let mut c = CMatrix::zero(rows, columns);
            let mut e = c.get_elements();

            for i in 0..rows {
                for j in 0..columns {
                    e[i][j] = element;
                }
            }
            c.set_elements(e);
            c
        }

        fn to_matrix2(self) -> Matrix2<T> {
            let mut m = Matrix2::zero();
            m.set_elements(self.get_elements());
            m
        }

        fn to_matrix3(self) -> Matrix3<T> {
            let mut m = Matrix3::zero();
            m.set_elements(self.get_elements());
            m
        }

        fn push(&mut self, v: Vec<T>) {
            let mut e = self.get_elements();
            e.push(v);
            self.rows += 1;
            self.set_elements(e);
        }

        fn pop(&mut self) {
            let mut e = self.get_elements();
            e.pop();
            self.rows -= 1;
            self.set_elements(e);
        }
    }

    impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Matrix<T> for CMatrix<T> {
        fn resize(&mut self) {
            let mut elems = self.get_elements();
            let rows = elems.len();
            let mut lens = vec![];
            for i in 0..rows {
                lens.push(elems[i].len());
            }
            let max_len = lens.iter().max().expect("Error in resize func!");
            for i in 0..rows {
                if elems[i].len() < *max_len {
                    elems[i].push(T::zero());
                }
            }
            self.rows = rows;
            self.columns = *max_len;
            self.elems = elems;
        }

        fn check_size(&self) {
            
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

        fn set_elements(&mut self, v: Vec<Vec<T>>) {
            self.rows = v.len();
            self.columns = v[0].len();
            self.elems = v;

            self.resize();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{CMatrix, CMatrixTrait, Matrix, Matrix2, Matrix23, Matrix3};

    #[test]
    fn cmatrix_i32_add_test() {
        let m = CMatrix::<i32>::one(2, 3);
        let m2 = CMatrix::<i32>::one(2, 3);

        assert_eq!((m + m2)[(0, 0)], 2);
    }

    #[test]
    fn cmatrix_f32_add_test() {
        let m = CMatrix::<f32>::one(2, 3);
        let m2 = CMatrix::<f32>::one(2, 3);

        assert_eq!((m + m2)[(0, 0)], 2.0);
    }

    #[test]
    fn cmatrix_i32_sub_test() {
        let m = CMatrix::<i32>::one(2, 3);
        let m2 = CMatrix::<i32>::one(2, 3);

        assert_eq!((m - m2)[(0, 0)], 0);
    }

    #[test]
    fn cmatrix_f32_sub_test() {
        let m = CMatrix::<f32>::one(2, 3);
        let m2 = CMatrix::<f32>::one(2, 3);

        assert_eq!((m - m2)[(0, 0)], 0.0);
    }

    #[test]
    #[should_panic]
    fn cmatrix_mul_panic_test() {
        let m = CMatrix::from_element(2, 3, 2);
        let m2 = CMatrix::from_element(2, 3, 2);

        assert_eq!((m * m2)[(0, 0)], 1);
    }

    #[test]
    fn cmatrix_cmatrix_mul_test() {
        let m = CMatrix::from_element(2, 3, 2);
        let m2 = CMatrix::from_element(3, 2, 2);

        assert_eq!((m.clone() * m2.clone())[(0, 0)], 12);
        assert_eq!((m.clone() * m2.clone())[(0, 1)], 12);
        assert_eq!((m.clone() * m2.clone())[(1, 0)], 12);
        assert_eq!((m.clone() * m2.clone())[(1, 1)], 12);
    }

    #[test]
    fn cmatrix_matrix2_mul_test() {
        let m = CMatrix::from_element(2, 2, 2);
        let m2 = Matrix2::from_element(2);

        assert_eq!((m.clone() * m2.clone())[(0, 0)], 8);
        assert_eq!((m.clone() * m2.clone())[(0, 1)], 8);
        assert_eq!((m.clone() * m2.clone())[(1, 0)], 8);
        assert_eq!((m.clone() * m2.clone())[(1, 1)], 8);
    }

    #[test]
    fn cmatrix_matrix3_mul_test() {
        let m = CMatrix::from_element(2, 3, 2);
        let m2 = Matrix3::from_element(2);

        let mul_result = m * m2;

        assert_eq!(mul_result[(0, 0)], 12);
        assert_eq!(mul_result[(0, 1)], 12);
        assert_eq!(mul_result[(0, 2)], 12);
        assert_eq!(mul_result[(1, 0)], 12);
        assert_eq!(mul_result[(1, 1)], 12);
        assert_eq!(mul_result[(1, 2)], 12);
    }

    #[test]
    fn cmatrix_transpose_test() {
        let m = CMatrix::from_vec_as_rows(2, vec![1, 2, 3, 4]);
        let mut m_clone = m.clone();
        m_clone.transpose();

        assert_eq!(m.get_columns(), m_clone.get_rows());
    }

    #[test]
    fn cmatrix_det_test() {
        let mut m = CMatrix::from_element(4, 4, 2);

        m.set_elements(vec![
            vec![2, 0, 1, 6],
            vec![3, 2, 8, 4],
            vec![4, 4, 4, 4],
            vec![8, 7, 9, 5],
        ]);

        assert_eq!(m.det(), 208);
    }

    #[test]
    #[should_panic]
    fn cmatrix_det_panic_test() {
        let mut m = CMatrix::from_element(4, 4, 2);

        m.set_elements(vec![vec![3, 2, 8, 4], vec![4, 4, 4, 4], vec![8, 7, 9, 5]]);

        assert_eq!(m.det(), 208);
    }
}
