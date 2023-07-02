extern crate mematrica;

use mematrica::*;

fn main() {
    let m2 = mat2![1, 2, 3, 4];
    let cm = cmat![1, 2, 3, 4];

    assert_eq!(m2, cm.to_matrix2());
}