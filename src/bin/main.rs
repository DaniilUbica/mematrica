use matrix_lib::*;

fn main() {
    let mut m = Matrix2::new(1,2,3,4);
    let mut m2 = CMatrix::from_vec_as_rows(2, vec![5,6]);

    m.set_elements(vec![vec![1, 2], vec![3, 4]]);
    m2.set_elements(vec![vec![5,6,0], vec![5,6,0]]);

    println!("{:?}", m*m2);
}
