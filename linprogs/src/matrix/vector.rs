use anyhow::{Result, anyhow};

pub trait Vector {
    fn values<'a>(&'a self) -> &'a [f64];

    fn values_mut<'a>(&'a mut self) -> &'a mut [f64];

    fn n(&self) -> usize;

    fn scale<'a>(&'a mut self, scale: f64) -> Result<()> {
        for v in self.values_mut() {
            *v *= scale
        }
        Ok(())
    }

    fn add<'a, T: Vector>(&'a mut self, to_add: T) -> Result<()> {
        if to_add.n() != self.n() {
            return Err(anyhow!(
                "to_add is wrong dimension: {} != {}",
                to_add.n(),
                self.n()
            ));
        }
        for (v, a) in self.values_mut().iter_mut().zip(to_add.values()) {
            *v += a
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct OwnedVector {
    values: Vec<f64>,
}

impl From<&[f64]> for OwnedVector {
    fn from(value: &[f64]) -> Self {
        Self {
            values: value.into(),
        }
    }
}

impl From<Vec<f64>> for OwnedVector {
    fn from(value: Vec<f64>) -> Self {
        Self { values: value }
    }
}

impl<const N: usize> From<[f64; N]> for OwnedVector {
    fn from(value: [f64; N]) -> Self {
        Self {
            values: value.into(),
        }
    }
}

impl Vector for OwnedVector {
    fn n(&self) -> usize {
        self.values.len()
    }

    fn values<'a>(&'a self) -> &'a [f64] {
        &self.values
    }

    fn values_mut<'a>(&'a mut self) -> &'a mut [f64] {
        self.values.as_mut_slice()
    }
}

#[derive(Debug)]
pub struct SliceMutRef<'a> {
    values: &'a mut [f64],
}

impl<'a> From<&'a mut [f64]> for SliceMutRef<'a> {
    fn from(value: &'a mut [f64]) -> Self {
        Self { values: value }
    }
}

impl<'a> Vector for SliceMutRef<'a> {
    fn values<'b>(&'b self) -> &'b [f64] {
        self.values
    }

    fn values_mut<'b>(&'b mut self) -> &'b mut [f64] {
        self.values
    }

    fn n(&self) -> usize {
        self.values.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::utils::assert_vec_eq;

    use super::*;

    mod owned {

        use super::*;

        #[test]
        fn from_vec() {
            let values: Vec<f64> = [1., 2., 3.].into();
            let expected = &[1., 2., 3.];

            let result = OwnedVector::from(values);
            assert_vec_eq(&result, expected);
        }
    }
}
