pub mod matrix3 {
    extern crate num;
    
    use crate::Error;
    use crate::cmatrix::cmatrix::CMatrix;
    pub use crate::matrix::matrix::Matrix;
    pub use crate::matrix23_trait::matrix23::Matrix23;

    use self::num::Num;
    use std::ops::{Mul, Sub};
    use std::{
        fs::OpenOptions,
        io::Read,
        ops::{Add, Index, IndexMut},
    };

    #[derive(Debug, Default, Clone)]
    pub struct Matrix3<T: Num + Default + Copy + PartialOrd + std::fmt::Debug> {
        rows: usize,
        columns: usize,
        elems: Vec<Vec<T>>,
    }

    impl<T: Num + Default + Copy + PartialOrd + std::fmt::Debug + std::convert::Into<f64>> Matrix3<T> {
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

    impl<T: Num + Default + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Matrix23<T>
        for Matrix3<T>
    {
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
            let file = OpenOptions::new()
                .read(true)
                .open(filename.clone());

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

            Ok(Matrix3::new(e[0], e[1], e[2], e[3], e[4], e[5], e[6], e[7], e[8]))
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
    }

    impl<T: Num + Default + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Matrix<T>
        for Matrix3<T>
    {
        fn resize(&mut self) {
            self.rows = self.elems.len();
            self.columns = self.elems.first().unwrap().len();
            if self.rows != self.columns && (self.columns != 3 || self.rows != 3) {
                panic!("Matrix3 have more than 3 elements in row or column!");
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
            } else {
                panic!("Can't make Matrix3 from this elements! Wrong size maybe?");
            }
        }
    }

    impl<T: Add<Output = T> + Num + Default + Clone + Copy + PartialOrd + std::fmt::Debug + std::convert::Into<f64>>
        Add<Matrix3<T>> for Matrix3<T>
    {
        type Output = Matrix3<T>;

        fn add(self, rhs: Matrix3<T>) -> Matrix3<T> {
            let mut v = self.elems.clone();
            let r = rhs.elems;

            for i in 0..self.rows {
                for j in 0..self.columns {
                    v[i][j] = v[i][j] + r[i][j];
                }
            }

            Matrix3 {
                rows: self.rows,
                columns: self.columns,
                elems: v,
            }
        }
    }

    impl<T: Add<Output = T> + Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>>
        Add<CMatrix<T>> for Matrix3<T>
    {
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
            } else {
                panic!("Can't fold this matrices: self.columns != rhs.columns || self.elems != rhs.elems");
            }

            Matrix3 {
                rows: self.rows,
                columns: self.columns,
                elems: v,
            }
        }
    }

    impl<T: Sub<Output = T> + Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>>
        Sub<Matrix3<T>> for Matrix3<T>
    {
        type Output = Matrix3<T>;

        fn sub(self, rhs: Matrix3<T>) -> Matrix3<T> {
            let mut v = self.elems.clone();
            let r = rhs.elems;

            for i in 0..self.rows {
                for j in 0..self.columns {
                    v[i][j] = v[i][j] - r[i][j];
                }
            }

            Matrix3 {
                rows: self.rows,
                columns: self.columns,
                elems: v,
            }
        }
    }

    impl<T: Sub<Output = T> + Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>>
        Sub<CMatrix<T>> for Matrix3<T>
    {
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
            } else {
                panic!("Can't fold this matrices: self.columns != rhs.columns || self.elems != rhs.elems");
            }

            Matrix3 {
                rows: self.rows,
                columns: self.columns,
                elems: v,
            }
        }
    }

    impl<T: Mul<Output = T> + Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>>
        Mul<Matrix3<T>> for Matrix3<T>
    {
        type Output = CMatrix<T>;

        fn mul(self, rhs: Matrix3<T>) -> CMatrix<T> {
            let mut m = self;
            m.multiplicate(rhs)
        }
    }

    impl<T: Mul<Output = T> + Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Mul<T>
        for Matrix3<T>
    {
        type Output = Matrix3<T>;

        fn mul(self, rhs: T) -> Matrix3<T> {
            let mut v = self.clone();

            for i in 0..self.rows {
                for j in 0..self.columns {
                    v[i][j] = v[i][j] * rhs;
                }
            }

            v
        }
    }

    impl<T: Mul<Output = T> + Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>>
        Mul<CMatrix<T>> for Matrix3<T>
    {
        type Output = CMatrix<T>;

        fn mul(self, rhs: CMatrix<T>) -> CMatrix<T> {
            let mut m = self;
            m.multiplicate(rhs)
        }
    }

    impl<T: Num + Default + Clone + Copy + PartialOrd + std::fmt::Debug + std::convert::Into<f64>> Index<(usize, usize)> for Matrix3<T> {
        type Output = T;

        fn index(&self, index: (usize, usize)) -> &Self::Output {
            &self.elems[index.0][index.1]
        }
    }

    impl<T: Num + Default + Clone + Copy + PartialOrd + std::fmt::Debug + std::convert::Into<f64>> IndexMut<(usize, usize)> for Matrix3<T> {
        fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
            &mut self.elems[index.0][index.1]
        }
    }

    impl<T: Num + Default + Clone + Copy + PartialOrd + std::fmt::Debug + std::convert::Into<f64>> Index<usize> for Matrix3<T> {
        type Output = Vec<T>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.elems[index]
        }
    }

    impl<T: Num + Default + Clone + Copy + PartialOrd + std::fmt::Debug + std::convert::Into<f64>> IndexMut<usize> for Matrix3<T> {
        fn index_mut(&mut self, index: usize) -> &mut Vec<T> {
            &mut self.elems[index]
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
