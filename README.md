# mematrica

## an easy to use library for working with matrices.

## Usage
```toml
[dependencies]
mematrica = "0.1"
```

## Examples
```rust
extern crate matrix_lib;

use matrix_lib::*;

fn main() {
    let mut matrix_2x2 = Matrix2::<i32>::new(1, 2, 3, 4); // creates a matrix 2x2 with elements 1, 2, 3, 4
    
    let double = matrix_2x2 * 2; // multiplies matrix on 2

    assert_eq!(vec![vec![2, 4], vec![6, 8]], double.get_elements()); 
}
```

### Using from_vec()
```rust
extern crate matrix_lib;

use matrix_lib::*;

fn main() {
    let mut matrix_2x2 = Matrix2::from_vec_as_rows(vec![1, 2]);

    assert_eq!(vec![vec![1, 2], vec![1, 2]], matrix_2x2.get_elements()); 
}
```

