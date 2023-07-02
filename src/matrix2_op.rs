extern crate num;
use crate::Matrix2;

use std::ops::Add;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::{Mul, Sub};

use self::num::Num;
pub use crate::cmatrix::cmatrix::CMatrix;
pub use crate::matrix::matrix::Matrix;
pub use crate::matrix23_trait::matrix23::Matrix23;

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Add<Matrix2<T>> for Matrix2<T> {
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

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Add<CMatrix<T>> for Matrix2<T> {
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

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Sub<Matrix2<T>> for Matrix2<T> {
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

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Sub<CMatrix<T>> for Matrix2<T> {
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

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Mul<Matrix2<T>> for Matrix2<T> {
    type Output = CMatrix<T>;

    fn mul(self, rhs: Matrix2<T>) -> CMatrix<T> {
        let m = self;
        m.multiplicate(rhs)
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Mul<T> for Matrix2<T> {
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

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Mul<CMatrix<T>> for Matrix2<T> {
    type Output = CMatrix<T>;

    fn mul(self, rhs: CMatrix<T>) -> CMatrix<T> {
        let m = self;
        m.multiplicate(rhs)
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Index<(usize, usize)> for Matrix2<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.elems[index.0][index.1]
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> IndexMut<(usize, usize)> for Matrix2<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.elems[index.0][index.1]
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Index<usize> for Matrix2<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> IndexMut<usize> for Matrix2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Vec<T> {
        &mut self.elems[index]
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> PartialEq for Matrix2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.check_size();
        other.check_size();
        if self.rows != other.rows || self.columns != other.columns {
            return false;
        }
        for i in 0..self.rows {
            for j in 0..self.columns {
                if self[i][j] != other[i][j] {
                    return false;
                }
            }
        }
        
        true
    }

    fn ne(&self, other: &Self) -> bool {
        self.check_size();
        other.check_size();
        if self.rows != other.rows || self.columns != other.columns {
            return true;
        }
        for i in 0..self.rows {
            for j in 0..self.columns {
                if self[i][j] != other[i][j] {
                    return true;
                }
            }
        }
        
        false
    }
}