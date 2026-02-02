mod raw;
mod vector;

pub use raw::RawMatrix;
pub use vector::Vector;

use crate::matrix::vector::{MutRefVector, RefVector};

pub trait Matrix {
    /// Returns all elements in the matrix as a flattened slice
    fn values(&self) -> &[f64];

    /// Returns the number of rows in the matrix
    fn m(&self) -> usize;

    /// Returns the number of columns in the matrix
    fn n(&self) -> usize;

    /// Returns a readonly view of a row in the matrix
    fn row<'a>(&'a self, i: usize) -> Option<RefVector<'a>>;

    /// Returns a writable view of a row in the matrix
    fn row_mut<'a>(&'a mut self, i: usize) -> Option<MutRefVector<'a>>;

    /// Returns the value of a given element in the matrix
    fn get(&self, i: usize, j: usize) -> Option<f64> {
        self.row(i)?.get(j)
    }
}

#[cfg(test)]
mod utils {
    use crate::matrix::{Matrix, Vector};
    const EPS: f64 = 1E-8;

    pub fn assert_vec_eq<T: Vector>(vec: &T, expected: &[f64]) -> () {
        assert_eq!(
            vec.n(),
            expected.len(),
            "actual and expected did not have same length"
        );

        for (ix, (a, e)) in vec.values().iter().zip(expected).enumerate() {
            assert!(
                (a - e).abs() < EPS,
                "vector element at index {} was not as expected.\n\texpected: {}\n\tactual: {}",
                ix,
                a,
                e
            )
        }
    }

    pub fn assert_mat_eq<M: Matrix>(mat: &M, expected: &[f64]) -> () {
        assert_eq!(
            mat.values().len(),
            expected.len(),
            "actual and expected did not have same length"
        );

        for (ix, (a, e)) in mat.values().iter().zip(expected).enumerate() {
            assert!(
                (a - e).abs() < EPS,
                "matrix element at row {} and column {} was not as expected.\n\texpected: {}\n\tactual: {}",
                ix / mat.n(),
                ix % mat.n(),
                e,
                a
            )
        }
    }
}
