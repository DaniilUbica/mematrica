extern crate mematrica;

use mematrica::*;

fn main() {
        // read from file/write to file

        let m = Matrix2::new(1, 2, 3, 4);
        m.to_file(String::from("./src/examples/file.txt"), ','); // in file now: 1,2,3,4
    
        let q = Matrix2::<i32>::from_file(String::from("./src/examples/file.txt"), ',');
    
        assert_eq!(vec![vec![1, 2], vec![3, 4]], q.get_elements());
    
        // or you can do something like this:
        let res = m.try_to_file(String::from("./src/examples/file.txt"), ',');
        let q = match res {
            Ok(_) => Matrix2::<i32>::from_file(String::from("./src/examples/file.txt"), ','),
            Err(_) => m.clone()*2,
        };
    
        assert_eq!(vec![vec![1, 2], vec![3, 4]], q.get_elements());
}