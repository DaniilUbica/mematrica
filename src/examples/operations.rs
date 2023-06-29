extern crate mematrica;

use mematrica::*;

fn main() {
    let m = Matrix2::new(1, 2, 3, 4);
    let q = Matrix2::new(1, 2, 3, 4);
    
    // operations with matrices
    let add = m.clone() + q.clone();
    let mul = m.clone() * q.clone();
    let sub = m.clone() - q.clone();

    // or with scalar
    let mul = m.clone() * 2;

    let w = Matrix3::from_element(4);
    // also you can use try_multiplicate:
    let mul = m.try_multiplicate(w).unwrap_err();
    assert_eq!(mul.0, String::from("Can't multiplicate this matrices: self.columns != rhs.rows"));
}