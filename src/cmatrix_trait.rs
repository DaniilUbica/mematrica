pub mod cmatrix_trait {
    extern crate num;
    
    use crate::{matrix::matrix::Error, Matrix2, Matrix3};
    use self::num::Num;

    pub trait CMatrixTrait<T: Num + Default + Clone + std::str::FromStr + std::cmp::PartialOrd + std::fmt::Debug + std::convert::Into<f64> + std::marker::Copy> {
        /// Creates a matrix of custom size with zero as its elements
        fn zero(rows: usize, columns: usize) -> Self;
        /// Creates a matrix of custom size with one as its elements
        fn one(rows: usize, columns: usize) -> Self;
        /// Creates an identity matrix of custom size
        fn identity(rows: usize, columns: usize) -> Self;
        /// Reads matrix elements from file
        fn from_file(filename: String, delimiter: char, rows: usize, columns: usize) -> Self
        where <T as std::str::FromStr>::Err: std::fmt::Debug;
        /// Try to read matrix elements from file
        fn try_from_file(filename: String, delimiter: char, rows: usize, columns: usize) -> Result<Self, Error>
        where <T as std::str::FromStr>::Err: std::fmt::Debug, Self : Sized;
        /// Creates a matrix of custom size from element
        fn from_element(rows: usize, columns: usize, e: T) -> Self;
        /// Creates a matrix from this vector, using that elements as columns
        fn from_vec_as_columns(columns: usize, v: Vec<T>) -> Self;
        /// Creates a matrix from this vector, using that elements as rows
        fn from_vec_as_rows(rows: usize, v: Vec<T>) -> Self;
        /// Creates a matrix from this element on its diagonal. All off-diagonal elements are set to 0
        fn from_diagonal(rows: usize, columns: usize, element: T) -> Self;
        /// Converts matrix to Matrix2
        fn to_matrix2(self) -> Matrix2<T>;
        /// Converts matrix to Matrix3
        fn to_matrix3(self) -> Matrix3<T>;
        /// Add a row to matrix
        fn push(&mut self, v: Vec<T>);
        /// Delete last row to matrix
        fn pop(&mut self);
    }

}