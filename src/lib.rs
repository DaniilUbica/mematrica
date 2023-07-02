//! # mematrica
//!
//! An easy to use library for working with matrices
//! ## Example
//! ```rust
//! extern crate mematrica;
//! 
//! use mematrica::*;
//!
//! let mut matrix_2x2 = Matrix2::new(1, 2, 3, 4);
//!   
//! let double = matrix_2x2 * 2;
//!
//! assert_eq!(vec![vec![2, 4], vec![6, 8]], double.get_elements());
//!
//! ```
mod matrix23_trait; 
mod cmatrix_trait;
mod matrix;
/// Matrix 2x2
pub mod matrix2; 
/// Matrix 3x3
pub mod matrix3;
/// Matrix of custom size
pub mod cmatrix;

/// Implementations of operations with matrix3
pub mod matrix3_op;
/// Implementations of operations with matrix2
pub mod matrix2_op;
/// Implementations of operations with cmatrix
pub mod cmatrix_op;
/// Macro for creations of matrices
pub mod macro_def;

pub use matrix23_trait::matrix23::Matrix23;
pub use cmatrix_trait::cmatrix_trait::CMatrixTrait;
pub use matrix::matrix::*;
pub use cmatrix::cmatrix::*;
pub use matrix3::matrix3::*;
pub use matrix2::matrix2::*;