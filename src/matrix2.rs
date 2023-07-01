pub mod matrix2 {
    extern crate num;
    use crate::CMatrixTrait;
    use crate::Error;

    use std::fs::OpenOptions;
    use std::io::Read;
    use std::ops::Add;
    use std::ops::Index;
    use std::ops::IndexMut;
    use std::ops::{Mul, Sub};

    use self::num::Num;
    pub use crate::cmatrix::cmatrix::CMatrix;
    pub use crate::matrix::matrix::Matrix;
    pub use crate::matrix23_trait::matrix23::Matrix23;

    #[derive(Debug, Default, Clone)]
    pub struct Matrix2<T: Num + Default + Clone + PartialOrd + std::fmt::Debug + std::convert::Into<f64> + std::str::FromStr> {
        rows: usize,
        columns: usize,
        elems: Vec<Vec<T>>,
    }

    impl<T: Num + Default + Clone + PartialOrd + std::fmt::Debug + std::convert::Into<f64> + std::str::FromStr> Matrix2<T> {
        pub fn new(m11: T, m12: T, m21: T, m22: T) -> Matrix2<T> {
            let e = vec![vec![m11, m12], vec![m21, m22]];
            Matrix2 {
                rows: 2,
                columns: 2,
                elems: e,
            }
        }
    }

    impl<T: Num + Default + Clone + PartialOrd + std::fmt::Debug + std::convert::Into<f64> + std::str::FromStr> Matrix23<T> for Matrix2<T> {
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

            Ok(Matrix2::new(e[0].clone(), e[1].clone(), e[2].clone(), e[3].clone()))
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

            let e = vec![vec![t[0].clone(), t[0].clone()], vec![t[1].clone(), t[1].clone()]];
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

            let e = vec![vec![t[0].clone(), t[1].clone()], vec![t[0].clone(), t[1].clone()]];
            Matrix2 {
                rows: 2,
                columns: 2,
                elems: e,
            }
        }

        fn to_cmatrix(&self) -> CMatrix<T> {
            let mut c = CMatrix::zero(self.rows, self.columns);
            c.set_elements(self.elems.clone());
            c
        }
    }

    impl<T: Num + Default + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Matrix<T>
        for Matrix2<T>
    {
        fn resize(&mut self) {
            if self.elems.len() != self.rows {
                if self.elems.first().unwrap().len() != self.columns {
                    let t = self.columns;
                    self.columns = self.rows;
                    self.rows = t;
                }
            }
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

    impl<T: Add<Output = T> + Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>>
        Add<Matrix2<T>> for Matrix2<T>
    {
        type Output = Matrix2<T>;

        fn add(self, rhs: Matrix2<T>) -> Matrix2<T> {
            let mut v = self.elems.clone();
            let r = rhs.elems;

            for i in 0..self.rows {
                for j in 0..self.columns {
                    v[i][j] = v[i][j] + r[i][j];
                }
            }

            Matrix2 {
                rows: self.rows,
                columns: self.columns,
                elems: v,
            }
        }
    }

    impl<T: Add<Output = T> + Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>>
        Add<CMatrix<T>> for Matrix2<T>
    {
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
            } else {
                panic!("Can't fold this matrices: self.columns != rhs.columns || self.elems != rhs.elems");
            }

            Matrix2 {
                rows: self.rows,
                columns: self.columns,
                elems: v,
            }
        }
    }

    impl<T: Sub<Output = T> + Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>>
        Sub<Matrix2<T>> for Matrix2<T>
    {
        type Output = Matrix2<T>;

        fn sub(self, rhs: Matrix2<T>) -> Matrix2<T> {
            let mut v = self.elems.clone();
            let r = rhs.elems;

            for i in 0..self.rows {
                for j in 0..self.columns {
                    v[i][j] = v[i][j] - r[i][j];
                }
            }

            Matrix2 {
                rows: self.rows,
                columns: self.columns,
                elems: v,
            }
        }
    }

    impl<T: Sub<Output = T> + Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>>
        Sub<CMatrix<T>> for Matrix2<T>
    {
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
            } else {
                panic!("Can't fold this matrices: self.columns != rhs.columns || self.elems != rhs.elems");
            }

            Matrix2 {
                rows: self.rows,
                columns: self.columns,
                elems: v,
            }
        }
    }

    impl<T: Mul<Output = T> + Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>>
        Mul<Matrix2<T>> for Matrix2<T>
    {
        type Output = CMatrix<T>;

        fn mul(self, rhs: Matrix2<T>) -> CMatrix<T> {
            let mut m = self;
            m.multiplicate(rhs)
        }
    }

    impl<T: Mul<Output = T> + Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Mul<T>
        for Matrix2<T>
    {
        type Output = Matrix2<T>;

        fn mul(self, rhs: T) -> Matrix2<T> {
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
        Mul<CMatrix<T>> for Matrix2<T>
    {
        type Output = CMatrix<T>;

        fn mul(self, rhs: CMatrix<T>) -> CMatrix<T> {
            let mut m = self;
            m.multiplicate(rhs)
        }
    }

    impl<T: Num + Default + Clone + PartialOrd + std::fmt::Debug + std::convert::Into<f64> + std::str::FromStr> Index<(usize, usize)> for Matrix2<T> {
        type Output = T;

        fn index(&self, index: (usize, usize)) -> &Self::Output {
            &self.elems[index.0][index.1]
        }
    }

    impl<T: Num + Default + Clone + PartialOrd + std::fmt::Debug + std::convert::Into<f64> + std::str::FromStr> IndexMut<(usize, usize)> for Matrix2<T> {
        fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
            &mut self.elems[index.0][index.1]
        }
    }

    impl<T: Num + Default + Clone + PartialOrd + std::fmt::Debug + std::convert::Into<f64> + std::str::FromStr> Index<usize> for Matrix2<T> {
        type Output = Vec<T>;

        fn index(&self, index: usize) -> &Self::Output {
            &self.elems[index]
        }
    }

    impl<T: Num + Default + Clone + PartialOrd + std::fmt::Debug + std::convert::Into<f64> + std::str::FromStr> IndexMut<usize> for Matrix2<T> {
        fn index_mut(&mut self, index: usize) -> &mut Vec<T> {
            &mut self.elems[index]
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
        let mut m = Matrix2::new(1, 2, 3, 4);
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
