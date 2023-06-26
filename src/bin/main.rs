extern crate matrix_lib;

use matrix_lib::*;

fn main() {
    let mut matrix_2x2 = Matrix2::<i32>::new(1, 2, 3, 4);
    
    let double = matrix_2x2 * 2;

    assert_eq!(vec![vec![2, 4], vec![6, 8]], double.get_elements());
}
