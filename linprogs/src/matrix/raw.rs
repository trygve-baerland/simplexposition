use anyhow::{Result, anyhow};

use crate::matrix::{
    Matrix, Vector,
    vector::{MutRefVector, MutVector, RefVector},
};

#[derive(Debug)]
pub struct RawMatrix {
    values: Vec<f64>,
    n: usize,
    m: usize,
}

impl RawMatrix {
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

    /// Scale a row in the matrix in-place.
    pub fn scale_row(&mut self, i: usize, scale: f64) -> Result<()> {
        match self.row_mut(i) {
            Some(mut row) => row.scale(scale),
            None => Err(anyhow!("row index {} is out of bounds", i)),
        }
    }

    pub fn add_row<T: Vector>(&mut self, i: usize, to_add: T) -> Result<()> {
        match self.row_mut(i) {
            Some(mut row) => row.add(to_add),
            None => Err(anyhow!("row index {} is out of bounds", i)),
        }
    }
}

impl Matrix for RawMatrix {
    /// Return the underlying values of the matrix
    fn values(&self) -> &[f64] {
        &self.values
    }
    fn m(&self) -> usize {
        self.m
    }

    fn n(&self) -> usize {
        self.n
    }

    fn row<'a>(&'a self, i: usize) -> Option<RefVector<'a>> {
        if i >= self.m {
            return None;
        }
        Some(self.values[self.m * i..(self.m * i + self.n)].into())
    }

    fn row_mut<'a>(&'a mut self, i: usize) -> Option<MutRefVector<'a>> {
        if i >= self.m {
            return None;
        }
        Some((&mut self.values[self.m * i..(self.m * i + self.n)]).into())
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::{
        utils::{self, assert_mat_eq, assert_vec_eq},
        vector::OwnedVector,
    };

    use num::ToPrimitive;

    use super::*;

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
        let expected = &[3., 4.];

        let row = mat.row(1);
        assert!(row.is_some());
        let row = row.unwrap();

        assert_vec_eq(&row, expected);
    }

    #[test]
    fn row_invalid_index() {
        let values = [1., 2., 3., 4.];

        let mat = RawMatrix::try_new(values.into(), 2, 2).unwrap();

        let row = mat.row(2);
        assert!(row.is_none());
    }

    #[test]
    fn scale_row() {
        let values = [1., 2., 3., 4.];

        let mut mat = RawMatrix::try_new(values.into(), 2, 2).unwrap();

        assert!(mat.scale_row(1, 3.).is_ok());

        let expected = [1., 2., 9., 12.];
        assert_mat_eq(&mat, &expected);
    }

    #[test]
    fn add_to_row() {
        let values = [1., 2., 3., 4.];

        let mut mat = RawMatrix::try_new(values.into(), 2, 2).unwrap();
        let to_add: OwnedVector = [3., 3.].into();

        assert!(mat.add_row(0, to_add).is_ok());

        let expected = [4., 5., 3., 4.];
        utils::assert_mat_eq(&mat, &expected);
    }

    #[test]
    fn scale_all_rows() {
        let values = [1., 2., 3., 4.];

        let mut mat = RawMatrix::try_new(values.into(), 2, 2).unwrap();

        assert!(
            mat.mutate_rows(|i, mut row| {
                let scale: f64 = 2.0 * (i + 1).to_f64().expect("should be valid");
                row.scale(scale)
            })
            .is_ok()
        );

        let expected = [2., 4., 12., 16.];
        assert_mat_eq(&mat, &expected);
    }

    #[test]
    fn pivot() {
        let values = [1., 2., 3., 4.];

        let mut mat = RawMatrix::try_new(values.into(), 2, 2).unwrap();

        assert!(mat.pivot(1, 0).is_ok());

        let expected = [0.0, 2. / 3., 1., 4. / 3.];
        assert_mat_eq(&mat, &expected);
    }
}
