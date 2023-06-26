pub mod cmatrix_trait {
    
    use num::Num;

    pub trait CMatrixTrait<T: Num + Default + Clone + std::str::FromStr> {
        /// Creates a matrix of custom size with zero as its elements
        fn zero(rows: usize, columns: usize) -> Self;
        /// Creates a matrix of custom size with one as its elements
        fn one(rows: usize, columns: usize) -> Self;
        /// Creates an identity matrix of custom size
        fn identity(rows: usize, columns: usize) -> Self;
        /// Reads matrix elements from file
        fn from_file(filename: String, delimiter: char, rows: usize, columns: usize) -> Self
        where <T as std::str::FromStr>::Err: std::fmt::Debug;

        /// Creates a matrix of custom size from element
        fn from_element(rows: usize, columns: usize, e: T) -> Self;
        /// Creates a matrix from this vector, using that elements as columns
        fn from_vec_as_columns(columns: usize, v: Vec<T>) -> Self;
        /// Creates a matrix from this vector, using that elements as rows
        fn from_vec_as_rows(rows: usize, v: Vec<T>) -> Self;
    }

}