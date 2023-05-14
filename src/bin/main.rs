use matrix_lib::*;

fn main() {
    let mut m = CMatrix::from_vec_as_rows(1, vec![5,6]);

    println!("{:?}", m.transpose());
}
