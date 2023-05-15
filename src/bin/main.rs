use matrix_lib::*;

fn main() {
    let mut m = Matrix2::new(1,2,3,4);
    let m2 = Matrix2::new(5,6,7,8);

    println!("{:?}", m*m2);
}
