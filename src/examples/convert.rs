extern crate mematrica;
extern crate mematrica;
use mematrica::*;

fn main() {
    let mut m = Matrix3::from_element(4); // creates matrix3
    m[0].push(3); // adds '3' to first row of matrix
    //m.det(); // panic! because m now is not square and seems like this: Matrix3 { rows: 3, columns: 3, elems: [[4, 4, 4, 3], [4, 4, 4], [4, 4, 4]] }
    // to find determinant you need to write something like that
    let mut m = m.to_cmatrix();
    m.push(vec![1,2,3,4]);
    let m = m.to_f64();
    m.det();
}