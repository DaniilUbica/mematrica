use matrix_lib::{*, cmatrix_trait::cmatrix_trait::CmatrixF32};

fn main() {
    let mut m = CMatrix::random(5, 5);
    println!("{:?}", m);
    println!("{:?}", m.det());
}
