mod raw;
mod vector;

use anyhow::{Result, anyhow};
pub use raw::RawMatrix;
pub use vector::{MutVector, Vector};

use crate::matrix::vector::VectorView;

const EPS: f64 = 1E-8;

pub trait Matrix {
    /// Returns the number of rows in the matrix
    fn m(&self) -> usize;

    /// Returns the number of columns in the matrix
    fn n(&self) -> usize;

    /// Returns the value of the i,j element in the matrix
    fn get(&self, i: usize, j: usize) -> Result<f64>;

    /// Returns the value of the i,j element in the matrix without
    /// checking bounds.
    fn get_unchecked(&self, i: usize, j: usize) -> f64;

    /// Returns the i'th row vector of the matrix
    fn row(&self, i: usize) -> Result<impl Vector>;

    /// Returns a copy of the i'th row of the matrix
    fn row_owned(&self, i: usize) -> Result<Vec<f64>> {
        let row = self.row(i)?;
        Ok((0..self.n()).map(|j| row.get_unchecked(j)).collect())
    }

    /// Returns an iterator over all rows in the matrix
    fn rows(&self) -> impl Iterator<Item = impl Vector> {
        (0..self.m()).map(|i| self.row(i).unwrap())
    }

    /// Returns a writable view of a row in the matrix
    fn row_mut(&mut self, i: usize) -> Result<impl MutVector>;

    /// Transform all rows in the matrix using some func
    fn mutate_rows<F>(&mut self, func: F) -> Result<()>
    where
        F: Fn(usize, &mut [f64]) -> Result<()>,
    {
        for i in 0..self.m() {
            func(i, self.row_mut(i).unwrap().as_mut_slice())?;
        }
        Ok(())
    }

    /// Pivot the matrix around element i,j.
    fn pivot(&mut self, i: usize, j: usize) -> Result<()> {
        let mut row = self.row_mut(i)?;
        let scale = row.get(j)?;

        if scale.abs() < EPS {
            return Err(anyhow!("cannot pivot around zero element"));
        }
        row.scale(1. / scale)?;
        drop(row);

        // get the pivot row and -column.
        let pivot_row: VectorView<_> = self
            .row_owned(i)
            .expect("we have already established that i is valid")
            .into();
        let pivot_element = pivot_row.get_unchecked(j);

        // Transform all other rows so that j is 0.
        self.mutate_rows(|ix, mut r| {
            if ix != i {
                r.add(&pivot_row * (-r.get_unchecked(j) / pivot_element))?;
            }
            Ok(())
        })
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
        for ix in 0..vec.n() {
            let a = vec.get_unchecked(ix);
            let e = expected[ix];
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
            mat.n() * mat.m(),
            expected.len(),
            "actual and expected did not have same length"
        );

        for ix in 0..mat.m() {
            for jx in 0..mat.n() {
                let a = mat.get_unchecked(ix, jx);
                let e = expected[jx + mat.n() * ix];
                assert!(
                    (a - e).abs() < EPS,
                    "matrix element at row {} and column {} was not as expected.\n\texpected: {}\n\tactual: {}",
                    ix / mat.n(),
                    ix % mat.n(),
                    e,
                    a
                );
            }
        }
    }
}
