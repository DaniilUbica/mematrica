extern crate mematrica;

use mematrica::*;

fn main() {
    let m2 = mat2![1, 2, 3, 4];
    // {1, 2}
    // {3, 4}
    let cm = cmat![1, 2; 3, 4];
    // {1, 2}
    // {3, 4}
    let m3 = Matrix3::from_diagonal(4);
    // {4, 0, 0}
    // {0, 4, 0}
    // {0, 0, 4}
    let m4 = Matrix2::from_element(2.0);
    // {2.0, 2.0}
    // {2.0, 2.0}
    let m5 = Matrix2::from_vec_as_columns(vec![1, 2]);
    // {1, 1}
    // {2, 2}
    let m5 = Matrix2::from_vec_as_columns(vec![1, 2, 3]);
    // panic! Too much elements in vector!

}