use anyhow::{Result, anyhow};

use crate::matrix::{Matrix, Vector, vector::MutVector};

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

    pub fn eye(n: usize) -> Self {
        let mut values = vec![0.; n * n];

        for i in 0..n {
            values[i + i * n] = 1.0;
        }

        Self { values, n, m: n }
    }

    pub fn try_from_rows<I, T>(rows: I) -> Result<RawMatrix>
    where
        T: Vector,
        I: IntoIterator<Item = T>,
    {
        let mut values = Vec::new();

        let mut n = None;
        let mut m = 0;

        for row in rows {
            if n.is_none() {
                n = Some(row.n());
            } else if let Some(n) = n
                && n != row.n()
            {
                return Err(anyhow!("mismatched row lengths"));
            }
            for j in 0..row.n() {
                values.push(row.get_unchecked(j));
            }
            m += 1;
        }
        Ok(Self {
            values,
            n: n.unwrap_or(0),
            m,
        })
    }

    /// Scale a row in the matrix in-place.
    pub fn scale_row(&mut self, i: usize, scale: f64) -> Result<()> {
        self.row_mut(i)?.scale(scale)
    }

    pub fn add_row<T: Vector>(&mut self, i: usize, to_add: T) -> Result<()> {
        self.row_mut(i)?.add(to_add)
    }
}

impl<const M: usize, const N: usize> From<[[f64; N]; M]> for RawMatrix {
    fn from(value: [[f64; N]; M]) -> Self {
        let values = value.into_iter().flatten().collect();

        Self { values, n: N, m: M }
    }
}

impl Matrix for RawMatrix {
    fn m(&self) -> usize {
        self.m
    }

    fn n(&self) -> usize {
        self.n
    }

    fn get(&self, i: usize, j: usize) -> Result<f64> {
        if i >= self.m() || j >= self.n() {
            return Err(anyhow!("invalid index of matrix: ({}, {})", i, j));
        }
        Ok(self.get_unchecked(i, j))
    }

    fn get_unchecked(&self, i: usize, j: usize) -> f64 {
        self.values[i * self.n() + j]
    }

    fn row(&self, i: usize) -> Result<impl Vector> {
        if i >= self.m {
            return Err(anyhow!("invalid row index {}", i));
        }
        Ok(&self.values[self.m * i..(self.m * i + self.n)])
    }

    fn row_mut(&mut self, i: usize) -> Result<impl MutVector> {
        if i >= self.m {
            return Err(anyhow!("invalid row index {}", i));
        }
        Ok(&mut self.values[self.m * i..(self.m * i + self.n)])
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::asserts::{self, assert_mat_eq, assert_vec_eq};

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
    fn try_from_rows() {
        let rows = [[1., 2., 3.], [4., 5., 6.]];

        let mat = RawMatrix::try_from_rows(rows).expect("should be valid");

        assert_mat_eq(&mat, &[1., 2., 3., 4., 5., 6.]);
    }

    #[test]
    fn from_array() {
        let values = [[1., 2., 3.], [4., 5., 6.]];

        let mat: RawMatrix = values.into();

        assert_mat_eq(&mat, &[1., 2., 3., 4., 5., 6.]);
    }

    #[test]
    fn eye3() {
        let mat = RawMatrix::eye(3);

        assert_mat_eq(&mat, &[1., 0., 0., 0., 1., 0., 0., 0., 1.]);
    }

    #[test]
    fn row_valid() {
        let values = [1., 2., 3., 4.];

        let mat = RawMatrix::try_new(values.into(), 2, 2).unwrap();
        let expected = &[3., 4.];

        let row = mat.row(1);
        assert!(row.is_ok());
        let row = row.unwrap();

        assert_vec_eq(&row, expected);
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
        assert_mat_eq(&mat, &expected);
    }

    #[test]
    fn add_to_row() {
        let values = [1., 2., 3., 4.];

        let mut mat = RawMatrix::try_new(values.into(), 2, 2).unwrap();
        let to_add = [3., 3.];

        assert!(mat.add_row(0, to_add).is_ok());

        let expected = [4., 5., 3., 4.];
        asserts::assert_mat_eq(&mat, &expected);
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
