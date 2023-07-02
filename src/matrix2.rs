pub mod matrix2 {
    extern crate num;
    use crate::CMatrixTrait;
    use crate::Error;

    use std::fs::OpenOptions;
    use std::io::Read;

    use self::num::Num;
    pub use crate::cmatrix::cmatrix::CMatrix;
    pub use crate::matrix::matrix::Matrix;
    pub use crate::matrix23_trait::matrix23::Matrix23;

    #[derive(Debug, Default, Clone, Eq)]
    pub struct Matrix2<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> {
        pub(crate) rows: usize,
        pub(crate) columns: usize,
        pub(crate) elems: Vec<Vec<T>>,
    }

    impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Matrix2<T> {
        pub fn new(m11: T, m12: T, m21: T, m22: T) -> Matrix2<T> {
            let e = vec![vec![m11, m12], vec![m21, m22]];
            Matrix2 {
                rows: 2,
                columns: 2,
                elems: e,
            }
        }
    }

    impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Matrix23<T> for Matrix2<T> {
        fn zero() -> Self {
            let e = vec![vec![T::zero(), T::zero()], vec![T::zero(), T::zero()]];
            Matrix2 {
                rows: 2,
                columns: 2,
                elems: e,
            }
        }

        fn one() -> Self {
            let e = vec![vec![T::one(), T::one()], vec![T::one(), T::one()]];
            Matrix2 {
                rows: 2,
                columns: 2,
                elems: e,
            }
        }

        fn identity() -> Self {
            let e = vec![vec![T::one(), T::zero()], vec![T::zero(), T::one()]];
            Matrix2 {
                rows: 2,
                columns: 2,
                elems: e,
            }
        }

        fn from_file(filename: String, delimiter: char) -> Self
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

            if e.len() < 4 {
                for _ in e.len()..4 {
                    e.push("0");
                }
            }

            let e: Vec<T> = e
                .iter()
                .map(|c| {
                    c.parse()
                        .expect("Can't parse file. Maybe some errors in delimiters?")
                })
                .collect();

            Matrix2::new(e[0].clone(), e[1].clone(), e[2].clone(), e[3].clone())
        }

        fn try_from_file(filename: String, delimiter: char) -> Result<Self, Error>
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

            if e.len() < 4 {
                for _ in e.len()..4 {
                    e.push("0");
                }
            }

            let e: Vec<T> = e
                .iter()
                .map(|c| {
                    c.parse()
                        .expect("Can't parse file. Maybe some errors in delimiters?")
                })
                .collect();

            Ok(Matrix2::new(
                e[0].clone(),
                e[1].clone(),
                e[2].clone(),
                e[3].clone(),
            ))
        }

        fn from_element(e: T) -> Self {
            let e = vec![vec![e.clone(), e.clone()], vec![e.clone(), e]];
            Matrix2 {
                rows: 2,
                columns: 2,
                elems: e,
            }
        }

        fn from_vec_as_columns(v: Vec<T>) -> Self {
            let mut t = v.clone();

            if v.len() > 2 {
                panic!("Too much elements in vector!");
            }

            while t.len() < 2 {
                t.push(T::zero());
            }

            let e = vec![
                vec![t[0].clone(), t[0].clone()],
                vec![t[1].clone(), t[1].clone()],
            ];
            Matrix2 {
                rows: 2,
                columns: 2,
                elems: e,
            }
        }

        fn from_vec_as_rows(v: Vec<T>) -> Self {
            let mut t = v.clone();

            if v.len() > 2 {
                panic!("Too much elements in vector!");
            }

            while t.len() < 2 {
                t.push(T::zero());
            }

            let e = vec![
                vec![t[0].clone(), t[1].clone()],
                vec![t[0].clone(), t[1].clone()],
            ];
            Matrix2 {
                rows: 2,
                columns: 2,
                elems: e,
            }
        }

        fn from_diagonal(element: T) -> Self {
            let mut c = Matrix2::zero();
            let mut e = c.get_elements();

            for i in 0..2 {
                for j in 0..2 {
                    e[i][j] = element;
                }
            }
            c.set_elements(e);
            c
        }

        fn to_cmatrix(self) -> CMatrix<T> {
            let mut c = CMatrix::zero(self.rows, self.columns);
            c.set_elements(self.elems.clone());
            c
        }
    }

    impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Matrix<T> for Matrix2<T> {
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
            let elems = self.get_elements();
            let i = elems.len();
            if i != 2 {
                panic!("Matrix2 have more or less than 2 elements in column!");
            }
            let mut q = vec![];
            for j in 0..i {
                q.push(elems[j].len());
            }
            for j in 0..q.len() {
                if q[j] != 2 {
                    panic!("Matrix2 have more or less than 2 elements in row!");
                }
            }
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
            if self.columns == v.len() && self.columns == v.first().unwrap().len() {
                self.elems = v;
                self.resize();
            } else {
                panic!("Can't make Matrix2 from this elements! Wrong size maybe?");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix, Matrix2, Matrix23};

    #[test]
    fn matrix2_i32_add_test() {
        let m = Matrix2::<i32>::one();
        let m2 = Matrix2::<i32>::one();

        assert_eq!((m + m2)[(0, 0)], 2);
    }

    #[test]
    fn matrix2_f32_add_test() {
        let m = Matrix2::<f32>::one();
        let m2 = Matrix2::<f32>::one();

        assert_eq!((m + m2)[(0, 0)], 2.0);
    }

    #[test]
    fn matrix2_i32_sub_test() {
        let m = Matrix2::<i32>::one();
        let m2 = Matrix2::<i32>::one();

        assert_eq!((m - m2)[(0, 0)], 0);
    }

    #[test]
    fn matrix2_f32_sub_test() {
        let m = Matrix2::<f32>::one();
        let m2 = Matrix2::<f32>::one();

        assert_eq!((m - m2)[(0, 0)], 0.0);
    }

    #[test]
    fn matrix2_mul_test() {
        let m = Matrix2::new(1, 2, 3, 4);
        let m2 = Matrix2::new(5, 6, 7, 8);

        assert_eq!((m.clone() * m2.clone())[(0, 0)], 19);
        assert_eq!((m.clone() * m2.clone())[(0, 1)], 22);
        assert_eq!((m.clone() * m2.clone())[(1, 0)], 43);
        assert_eq!((m.clone() * m2.clone())[(1, 1)], 50);

        assert_eq!((m.clone().multiplicate(m2.clone()))[(0, 0)], 19);
        assert_eq!((m.clone().multiplicate(m2.clone()))[(0, 1)], 22);
        assert_eq!((m.clone().multiplicate(m2.clone()))[(1, 0)], 43);
        assert_eq!((m.multiplicate(m2))[(1, 1)], 50);
    }

    #[test]
    fn matrix2_det_test() {
        let m = Matrix2::new(5, 6, 1, 4);

        assert_eq!(m.det(), 14);
    }
}
