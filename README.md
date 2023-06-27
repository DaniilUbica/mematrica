# mematrica [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url]

![mematrica_logo](https://github.com/DaniilUbica/mematrica/assets/102466617/a298c590-78c3-4ae8-89fa-62a8e0a2de28)

## an easy to use library for working with matrices.

## Features
* Matrix-scalar addition, substraction, multiplication
* Matrix-matrix addition, substraction, multiplication
* Matrix indexing
* Matrix determinant search
* Inverse matrix search
* Matrix transpose
* Matrix from file reading
* Matrix to file writing

## Usage
```toml
[dependencies]
mematrica = "0.1.1"
```

## Examples
```rust
extern crate mematrica;

use mematrica::*;

fn main() {
    let mut matrix_2x2 = Matrix2::new(1, 2, 3, 4); // creates a matrix 2x2 with elements 1, 2, 3, 4
    
    let double = matrix_2x2 * 2; // multiplies matrix on 2

    assert_eq!(vec![vec![2, 4], vec![6, 8]], double.get_elements()); 
}
```

### Using from_vec()
```rust
extern crate mematrica;

use mematrica::*;

fn main() {
    let matrix_2x2 = Matrix2::from_vec_as_rows(vec![1, 2]);

    assert_eq!(vec![vec![1, 2], vec![1, 2]], matrix_2x2.get_elements()); 
}
```

### Other ways to create matrix
- `zero`: creates a matrix with zeros as its elements
- `one`: creates a matrix with ones as its elements
- `identity`: creates an identity matrix
- `from_file`: reads matrix from file
- `from_element`: creates a matrix from element
- `from_vec`: creates matrix from vector as its rows or columns (`from_vec_as_rows` or `from vec_as_columns`)

### Get element by index
```rust
extern crate mematrica;

use mematrica::*;

fn main() {
    let mut matrix_2x2 = Matrix2::new(1, 2, 3, 4);

    assert_eq!(1, matrix_2x2[(0, 0)]);

    // change element
    matrix[(0, 0)] = 2;
    assert_eq!(2, matrix_2x2[(0, 0)]);
}
```
### Elementary operations
```rust
extern crate mematrica;

use mematrica::*;

fn main() {
    let m1 = CMatrix::from_element(2, 2, 4);
    let m2 = Matrix2::from_element(2);

    assert_eq!(vec![vec![16, 16], vec![16, 16]], (m1*m2).get_elements()); //also can use '+' '-'
}
```

[documentation-img]: https://docs.rs/mematrica/badge.svg
[documentation-url]: https://docs.rs/mematrica
[package-img]: https://img.shields.io/crates/v/mematrica.svg
[package-url]: https://crates.io/crates/mematrica
