mod raw;
mod vector;

use anyhow::{Result, anyhow};
pub use raw::RawMatrix;
pub use vector::{MutVector, Vector};

use crate::matrix::vector::{MutRefVector, OwnedVector};

const EPS: f64 = 1E-8;

pub trait Matrix {
    /// Returns all elements in the matrix as a flattened slice
    fn values(&self) -> &[f64];

    /// Returns the number of rows in the matrix
    fn m(&self) -> usize;

    /// Returns the number of columns in the matrix
    fn n(&self) -> usize;

    /// Returns a readonly view of a row in the matrix
    fn row(&self, i: usize) -> Option<impl Vector>;

    /// Returns a copy of the i'th row of the matrix
    fn row_owned(&self, i: usize) -> Result<OwnedVector> {
        if let Some(row) = self.row(i) {
            return Ok(row.values().into());
        }
        Err(anyhow!("invalid row index {}", i))
    }

    /// Returns an iterator over all rows in the matrix
    fn rows(&self) -> impl Iterator<Item = impl Vector> {
        (0..self.m()).map(|i| self.row(i).unwrap())
    }

    /// Returns a writable view of a row in the matrix
    fn row_mut(&mut self, i: usize) -> Option<impl MutVector>;

    /// Transform all rows in the matrix using some func
    fn mutate_rows<F>(&mut self, func: F) -> Result<()>
    where
        F: Fn(usize, MutRefVector<'_>) -> Result<()>,
    {
        for i in 0..self.m() {
            func(i, self.row_mut(i).unwrap().values_mut().into())?;
        }
        Ok(())
    }

    /// Pivot the matrix around element i,j.
    fn pivot(&mut self, i: usize, j: usize) -> Result<()> {
        // Scale the given row so that i,j is 1.0
        if let Some(mut row) = self.row_mut(i) {
            if let Some(scale) = row.get(j) {
                if scale.abs() < EPS {
                    return Err(anyhow!("cannot pivot around zero element"));
                }
                row.scale(1. / scale)?;
            } else {
                return Err(anyhow!("invalid column index {}", j));
            }
        } else {
            return Err(anyhow!("invalid row index {}", i));
        }

        // get the pivot row and -column.
        let pivot_row = self
            .row_owned(i)
            .expect("we have already established that i is valid");
        let pivot_element = pivot_row.get_unchecked(j);

        // Transform all other rows so that j is 0.
        self.mutate_rows(|ix, mut row| {
            if ix != i {
                row.add(&pivot_row * (-row.get_unchecked(j) / pivot_element))?;
            }
            Ok(())
        })
    }

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
