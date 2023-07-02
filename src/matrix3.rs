pub mod matrix3 {
    extern crate num;

    use crate::cmatrix::cmatrix::CMatrix;
    pub use crate::matrix::matrix::Matrix;
    pub use crate::matrix23_trait::matrix23::Matrix23;
    use crate::{Error, CMatrixTrait};

    use self::num::Num;
    use std::{fs::OpenOptions, io::Read};

    #[derive(Debug, Default, Clone, Eq)]
    pub struct Matrix3<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> {
        pub(crate) rows: usize,
        pub(crate) columns: usize,
        pub(crate) elems: Vec<Vec<T>>,
    }

    impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Matrix3<T> {
        pub fn new(
            m11: T,
            m12: T,
            m13: T,
            m21: T,
            m22: T,
            m23: T,
            m31: T,
            m32: T,
            m33: T,
        ) -> Matrix3<T> {
            let e = vec![
                vec![m11, m12, m13],
                vec![m21, m22, m23],
                vec![m31, m32, m33],
            ];
            Matrix3 {
                rows: 3,
                columns: 3,
                elems: e,
            }
        }
    }

    impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Matrix23<T> for Matrix3<T> {
        fn zero() -> Self {
            let e = vec![
                vec![T::zero(), T::zero(), T::zero()],
                vec![T::zero(), T::zero(), T::zero()],
                vec![T::zero(), T::zero(), T::zero()],
            ];
            Matrix3 {
                rows: 3,
                columns: 3,
                elems: e,
            }
        }

        fn one() -> Self {
            let e = vec![
                vec![T::one(), T::one(), T::one()],
                vec![T::one(), T::one(), T::one()],
                vec![T::one(), T::one(), T::one()],
            ];

            Matrix3 {
                rows: 3,
                columns: 3,
                elems: e,
            }
        }

        fn identity() -> Self {
            let e = vec![
                vec![T::one(), T::zero(), T::zero()],
                vec![T::zero(), T::one(), T::zero()],
                vec![T::zero(), T::zero(), T::one()],
            ];

            Matrix3 {
                rows: 3,
                columns: 3,
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

            if e.len() < 9 {
                for _ in e.len()..9 {
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

            Matrix3::new(e[0], e[1], e[2], e[3], e[4], e[5], e[6], e[7], e[8])
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

            if e.len() < 9 {
                for _ in e.len()..9 {
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

            Ok(Matrix3::new(
                e[0], e[1], e[2], e[3], e[4], e[5], e[6], e[7], e[8],
            ))
        }

        fn from_element(e: T) -> Self {
            let e = vec![vec![e, e, e], vec![e, e, e], vec![e, e, e]];
            Matrix3 {
                rows: 3,
                columns: 3,
                elems: e,
            }
        }

        fn from_vec_as_columns(v: Vec<T>) -> Self {
            let mut t = v.clone();

            if v.len() > 3 {
                panic!("Too much elements in vector!");
            }

            while t.len() < 3 {
                t.push(T::zero());
            }

            let e = vec![
                vec![t[0], t[0], t[0]],
                vec![t[1], t[1], t[1]],
                vec![t[2], t[2], t[2]],
            ];
            Matrix3 {
                rows: 3,
                columns: 3,
                elems: e,
            }
        }

        fn from_vec_as_rows(v: Vec<T>) -> Self {
            let mut t = v.clone();

            if v.len() > 3 {
                panic!("Too much elements in vector!");
            }

            while t.len() < 3 {
                t.push(T::zero());
            }

            let e = vec![
                vec![t[0], t[1], t[2]],
                vec![t[0], t[1], t[2]],
                vec![t[0], t[1], t[2]],
            ];
            Matrix3 {
                rows: 3,
                columns: 3,
                elems: e,
            }
        }

        fn from_diagonal(element: T) -> Self {
            let mut c = Matrix3::zero();
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

    impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Matrix<T> for Matrix3<T> {
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
            if i != 3 {
                panic!("Matrix3 have more or less than 3 elements in column!");
            }
            let mut q = vec![];
            for j in 0..i {
                q.push(elems[j].len());
            }
            for j in 0..q.len() {
                if q[j] != 3 {
                    panic!("Matrix3 have more or less than 3 elements in row!");
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
                panic!("Can't make Matrix3 from this elements! Wrong size maybe?");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix, Matrix23, Matrix3};

    #[test]
    fn matrix3_i32_add_test() {
        let m = Matrix3::<i32>::one();
        let m2 = Matrix3::<i32>::one();

        assert_eq!((m + m2)[(0, 0)], 2);
    }

    #[test]
    fn matrix3_f32_add_test() {
        let m = Matrix3::<f32>::one();
        let m2 = Matrix3::<f32>::one();

        assert_eq!((m + m2)[(0, 0)], 2.0);
    }

    #[test]
    fn matrix3_i32_sub_test() {
        let m = Matrix3::<i32>::one();
        let m2 = Matrix3::<i32>::one();

        assert_eq!((m - m2)[(0, 0)], 0);
    }

    #[test]
    fn matrix3_f32_sub_test() {
        let m = Matrix3::<f32>::one();
        let m2 = Matrix3::<f32>::one();

        assert_eq!((m - m2)[(0, 0)], 0.0);
    }

    #[test]
    fn matrix3_mul_test() {
        let m = Matrix3::new(1, 2, 3, 4, 5, 6, 7, 8, 9);
        let m2 = Matrix3::new(10, 11, 12, 13, 14, 15, 16, 17, 18);

        assert_eq!((m.clone() * m2.clone())[(0, 0)], 84);
        assert_eq!((m.clone() * m2.clone())[(0, 1)], 90);
        assert_eq!((m.clone() * m2.clone())[(0, 2)], 96);
        assert_eq!((m.clone() * m2.clone())[(1, 0)], 201);
        assert_eq!((m.clone() * m2.clone())[(1, 1)], 216);
        assert_eq!((m.clone() * m2.clone())[(1, 2)], 231);
        assert_eq!((m.clone() * m2.clone())[(2, 0)], 318);
        assert_eq!((m.clone() * m2.clone())[(2, 1)], 342);
        assert_eq!((m.clone() * m2.clone())[(2, 2)], 366);
    }

    #[test]
    fn matrix3_det_test() {
        let m = Matrix3::new(1, 4, 6, 2, 1, 2, 8, 3, 2);

        assert_eq!(m.det(), 32);
    }
}
