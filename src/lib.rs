//! # matrix_lib
//!
//! An easy to use library for working with matrices

mod matrix23_trait; 
mod cmatrix_trait;
mod matrix;
/// Matrix 2x2
pub mod matrix2; 
/// Matrix 3x3
pub mod matrix3;
/// Matrix of custom size
pub mod cmatrix; 

pub use matrix23_trait::matrix23::Matrix23;
pub use cmatrix_trait::cmatrix_trait::CMatrixTrait;
pub use matrix::matrix::*;
pub use matrix3::matrix3::Matrix3;
pub use matrix2::matrix2::*;