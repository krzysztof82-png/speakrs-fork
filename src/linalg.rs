//! One BLAS backend, selected per target in Cargo.toml.
pub(crate) use ndarray_linalg::{Eigh, Inverse, UPLO, error::LinalgError};
