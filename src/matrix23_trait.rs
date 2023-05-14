pub mod matrix23 {
    use num::Num;

    pub trait Matrix23<T: Num + Default + Clone> {
        fn zero() -> Self;
        fn one() -> Self;
        fn from_element(e: T) -> Self;
        fn from_vec_as_columns(v: Vec<T>) -> Self;
        fn from_vec_as_rows(v: Vec<T>) -> Self;
    }
}