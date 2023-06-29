pub mod matrix23 {
    extern crate num;

    use crate::matrix::matrix::Error;
    use self::num::Num;

    pub trait Matrix23<T: Num + Default + Clone + std::str::FromStr> {
        /// Creates a matrix with zero as its elements
        fn zero() -> Self;
        /// Creates a matrix with one as its elements
        fn one() -> Self;
        /// Creates an identity matrix
        fn identity() -> Self;
        /// Read matrix from file
        fn from_file(filename: String, delimiter: char) -> Self
        where <T as std::str::FromStr>::Err: std::fmt::Debug;
        /// Try to read matrix elements from file
        fn try_from_file(filename: String, delimiter: char) -> Result<Self, Error>
        where <T as std::str::FromStr>::Err: std::fmt::Debug, Self : Sized;
        /// Creates a matrix from element
        fn from_element(e: T) -> Self;
        /// Creates a matrix from vector, using that elements as columns
        fn from_vec_as_columns(v: Vec<T>) -> Self;
        /// Creates a matrix from vector, using that elements as rows
        fn from_vec_as_rows(v: Vec<T>) -> Self;
    }
}