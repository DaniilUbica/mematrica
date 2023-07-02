extern crate num;

pub use crate::cmatrix_trait::cmatrix_trait::CMatrixTrait;
pub use crate::matrix::matrix::Matrix;
use crate::matrix2::matrix2::Matrix2;
use crate::matrix3::matrix3::Matrix3;
use crate::CMatrix;

use self::num::Num;
use std::ops::{Mul, Sub};
use std::ops::{Add, Index, IndexMut};

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Add<Matrix3<T>> for CMatrix<T> {
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
        } else {
            panic!("Can't fold this matrices: self.columns != rhs.columns || self.rows != rhs.rows");
        }

        CMatrix {
            rows: self.rows,
            columns: self.columns,
            elems: v,
        }
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Add<Matrix2<T>> for CMatrix<T> {
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
        } else {
            panic!("Can't fold this matrices: self.columns != rhs.columns || self.rows != rhs.rows");
        }

        CMatrix {
            rows: self.rows,
            columns: self.columns,
            elems: v,
        }
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Add<CMatrix<T>> for CMatrix<T> {
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
        } else {
            panic!("Can't fold this matrices: self.columns != rhs.columns || self.rows != rhs.rows");
        }

        CMatrix {
            rows: self.rows,
            columns: self.columns,
            elems: v,
        }
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Sub<Matrix3<T>> for CMatrix<T> {
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
        } else {
            panic!("Can't fold this matrices: self.columns != rhs.columns || self.rows != rhs.rows");
        }

        CMatrix {
            rows: self.rows,
            columns: self.columns,
            elems: v,
        }
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Sub<Matrix2<T>> for CMatrix<T> {
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
        } else {
            panic!("Can't fold this matrices: self.columns != rhs.columns || self.rows != rhs.rows");
        }

        CMatrix {
            rows: self.rows,
            columns: self.columns,
            elems: v,
        }
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Sub<CMatrix<T>> for CMatrix<T> {
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
        } else {
            panic!("Can't fold this matrices: self.columns != rhs.columns || self.rows != rhs.rows");
        }

        CMatrix {
            rows: self.rows,
            columns: self.columns,
            elems: v,
        }
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Mul<Matrix3<T>> for CMatrix<T> {
    type Output = CMatrix<T>;

    fn mul(self, rhs: Matrix3<T>) -> CMatrix<T> {
        let m = self;
        m.multiplicate(rhs)
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Mul<T> for CMatrix<T> {
    type Output = CMatrix<T>;

    fn mul(self, rhs: T) -> CMatrix<T> {
        let mut v = self.clone();

        for i in 0..self.rows {
            for j in 0..self.columns {
                v[i][j] = v[i][j] * rhs;
            }
        }

        v
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Mul<Matrix2<T>> for CMatrix<T> {
    type Output = CMatrix<T>;

    fn mul(self, rhs: Matrix2<T>) -> CMatrix<T> {
        let m = self;
        m.multiplicate(rhs)
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Mul<CMatrix<T>> for CMatrix<T> {
    type Output = CMatrix<T>;

    fn mul(self, rhs: CMatrix<T>) -> CMatrix<T> {
        let m = self;
        m.multiplicate(rhs)
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Index<(usize, usize)> for CMatrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.elems[index.0][index.1]
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> IndexMut<(usize, usize)> for CMatrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.elems[index.0][index.1]
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> Index<usize> for CMatrix<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> IndexMut<usize> for CMatrix<T> {
    fn index_mut(&mut self, index: usize) -> &mut Vec<T> {
        &mut self.elems[index]
    }
}

impl<T: Num + Default + Clone + Copy + PartialOrd + std::str::FromStr + std::fmt::Debug + std::convert::Into<f64>> PartialEq for CMatrix<T> {
    fn eq(&self, other: &Self) -> bool {
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
