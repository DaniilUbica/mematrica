pub mod cmatrix_trait {
    use num::Num;

    pub trait CMatrixTrait<T: Num + Default + Clone + std::str::FromStr> {
        fn zero(rows: usize, columns: usize) -> Self;
        fn one(rows: usize, columns: usize) -> Self;
        fn identity(rows: usize, columns: usize) -> Self;
        fn from_file(filename: String, delimiter: char, rows: usize, columns: usize) -> Self
        where <T as std::str::FromStr>::Err: std::fmt::Debug;

        fn from_element(rows: usize, columns: usize, e: T) -> Self;
        fn from_vec_as_columns(columns: usize, v: Vec<T>) -> Self;
        fn from_vec_as_rows(rows: usize, v: Vec<T>) -> Self;
    }

    //for tests//
    pub trait CmatrixF32 {
        fn random(rows: usize, columns: usize) -> Self;
    }
}