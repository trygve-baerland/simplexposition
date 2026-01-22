use anyhow::{Result, anyhow};

#[derive(Debug)]
pub struct RawMatrix<'a> {
    values: &'a [f64],
    n: usize,
    m: usize,
}

impl<'a> RawMatrix<'a> {
    pub fn try_new(values: &'a [f64], n: usize, m: usize) -> Result<RawMatrix<'a>> {
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

    /// Return the i'th row of the matrix
    pub fn row(&self, i: usize) -> Result<&[f64]> {
        if i >= self.m {
            return Err(anyhow!("row index {} out of bounds", i));
        }
        Ok(&self.values[self.m * i..(self.m * i + self.n)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1E-8;

    #[test]
    fn try_new_valid() {
        let values: &[f64] = &[1., 2., 3., 4.];
        assert!(RawMatrix::try_new(values, 2, 2).is_ok());
    }

    #[test]
    fn try_new_invalid() {
        let values: &[f64] = &[1., 2., 3., 4.];
        assert!(RawMatrix::try_new(values, 3, 1).is_err());
    }

    #[test]
    fn row_valid() {
        let values: &[f64] = &[1., 2., 3., 4.];

        let mat = RawMatrix::try_new(values, 2, 2).unwrap();
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
        let values: &[f64] = &[1., 2., 3., 4.];

        let mat = RawMatrix::try_new(values, 2, 2).unwrap();

        let row = mat.row(2);
        assert!(row.is_err());
    }
}
