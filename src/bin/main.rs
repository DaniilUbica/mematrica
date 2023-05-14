use matrix_lib::*;

fn main() {
    let mut m = CMatrix::from_vec_as_columns(4, vec![4, 6, 8, 4, 6]);
    println!("{:?}", m.transpose());
}
