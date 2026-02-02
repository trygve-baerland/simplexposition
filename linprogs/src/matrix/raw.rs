use anyhow::{Result, anyhow};

use crate::matrix::vector::SliceMutRef;

#[derive(Debug)]
pub struct RawMatrix {
    values: Vec<f64>,
    n: usize,
    m: usize,
}

impl<'a> RawMatrix {
    pub fn try_new(values: Vec<f64>, n: usize, m: usize) -> Result<RawMatrix> {
        if values.len() != n * m {
            return Err(anyhow!(
                "dimension mismatch: {} != {} * {}",
                values.len(),
                n,
                m
            ));
        }
        Ok(RawMatrix { values, n, m })
    }

    /// Return the number of columns in the matrix
    pub fn n(&self) -> usize {
        self.n
    }

    /// Return the number of rows in the matrix
    pub fn m(&self) -> usize {
        self.m
    }

    /// Return the underlying values of the matrix
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// Return the i'th row of the matrix
    pub fn row(&self, i: usize) -> Result<&[f64]> {
        if i >= self.m {
            return Err(anyhow!("row index {} out of bounds", i));
        }
        Ok(&self.values[self.m * i..(self.m * i + self.n)])
    }

    /// Return a mut reference to the i'th row.
    fn row_mut(&'_ mut self, i: usize) -> Result<SliceMutRef<'_>> {
        if i >= self.m {
            return Err(anyhow!("row index {} out of bounds", i));
        }
        Ok((&mut self.values[self.m * i..(self.m * i + self.n)]).into())
    }

    /// Scale a row in the matrix in-place.
    pub fn scale_row(&mut self, i: usize, scale: f64) -> Result<()> {
        self.row_mut(i)?.scale(scale)
    }

    pub fn add_row(&mut self, i: usize, to_add: &[f64]) -> Result<()> {
        if to_add.len() != self.n {
            return Err(anyhow!(
                "to_add is wrong dimension: {} != {}",
                to_add.len(),
                self.n
            ));
        }

        for (v, a) in self.row_mut(i)?.iter_mut().zip(to_add) {
            *v += a
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::utils;

    use super::*;

    const EPS: f64 = 1E-8;

    #[test]
    fn try_new_valid() {
        let values = [1., 2., 3., 4.];
        assert!(RawMatrix::try_new(values.into(), 2, 2).is_ok());
    }

    #[test]
    fn try_new_invalid() {
        let values = [1., 2., 3., 4.];
        assert!(RawMatrix::try_new(values.into(), 3, 1).is_err());
    }

    #[test]
    fn row_valid() {
        let values = [1., 2., 3., 4.];

        let mat = RawMatrix::try_new(values.into(), 2, 2).unwrap();
        let expected = [3., 4.];

        let row = mat.row(1);
        assert!(row.is_ok());
        let row = row.unwrap();

        assert!(
            row.iter()
                .zip(expected.iter())
                .all(|(a, e)| (a - e).abs() < EPS)
        );
    }

    #[test]
    fn row_invalid_index() {
        let values = [1., 2., 3., 4.];

        let mat = RawMatrix::try_new(values.into(), 2, 2).unwrap();

        let row = mat.row(2);
        assert!(row.is_err());
    }

    #[test]
    fn scale_row() {
        let values = [1., 2., 3., 4.];

        let mut mat = RawMatrix::try_new(values.into(), 2, 2).unwrap();

        assert!(mat.scale_row(1, 3.).is_ok());

        let expected = [1., 2., 9., 12.];
        assert!(
            mat.values
                .iter()
                .zip(expected)
                .all(|(a, e)| (a - e).abs() < EPS)
        );
    }

    #[test]
    fn add_to_row() {
        let values = [1., 2., 3., 4.];

        let mut mat = RawMatrix::try_new(values.into(), 2, 2).unwrap();
        let to_add = &[3., 3.];

        assert!(mat.add_row(0, to_add).is_ok());

        let expected = [4., 5., 3., 4.];
        utils::assert_mat_eq(&mat, &expected);
    }
}
