pub mod matrix23 {
    use num::Num;

    pub trait Matrix23<T: Num + Default + Clone + std::str::FromStr> {
        fn zero() -> Self;
        fn one() -> Self;
        fn identity() -> Self;
        fn from_file(filename: String, delimiter: char) -> Self
        where <T as std::str::FromStr>::Err: std::fmt::Debug;
        fn from_element(e: T) -> Self;
        fn from_vec_as_columns(v: Vec<T>) -> Self;
        fn from_vec_as_rows(v: Vec<T>) -> Self;
    }
}