use anyhow::{Result, anyhow};

pub trait Vector {
    fn values(&self) -> &[f64];

    fn n(&self) -> usize;

    fn get(&self, index: usize) -> Option<f64> {
        self.values().get(index).copied()
    }

    fn get_unchecked(&self, index: usize) -> f64 {
        self.values()[index]
    }
}

pub trait MutVector: Vector {
    fn values_mut(&mut self) -> &mut [f64];

    fn scale(&mut self, scale: f64) -> Result<()> {
        for v in self.values_mut() {
            *v *= scale;
        }
        Ok(())
    }

    fn add<T: Vector>(&mut self, to_add: T) -> Result<()> {
        if to_add.n() != self.n() {
            return Err(anyhow!(
                "to_add is wrong dimension: {} != {}",
                to_add.n(),
                self.n()
            ));
        }
        for (v, a) in self.values_mut().iter_mut().zip(to_add.values()) {
            *v += a;
        }
        Ok(())
    }
}

#[allow(unused)]
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

    fn values(&self) -> &[f64] {
        &self.values
    }
}

impl MutVector for OwnedVector {
    fn values_mut(&mut self) -> &mut [f64] {
        self.values.as_mut_slice()
    }
}

#[derive(Debug)]
pub struct MutRefVector<'a> {
    values: &'a mut [f64],
}

impl<'a> From<&'a mut [f64]> for MutRefVector<'a> {
    fn from(value: &'a mut [f64]) -> Self {
        Self { values: value }
    }
}

impl<'a> Vector for MutRefVector<'a> {
    fn values(&self) -> &[f64] {
        self.values
    }
    fn n(&self) -> usize {
        self.values.len()
    }
}

impl<'a> MutVector for MutRefVector<'a> {
    fn values_mut(&mut self) -> &mut [f64] {
        self.values
    }
}

#[derive(Debug)]
pub struct RefVector<'a> {
    values: &'a [f64],
}

impl<'a> From<&'a [f64]> for RefVector<'a> {
    fn from(value: &'a [f64]) -> Self {
        Self { values: value }
    }
}

impl<'a> Vector for RefVector<'a> {
    fn values(&self) -> &[f64] {
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

        #[test]
        fn from_slice() {
            let values = &[1., 2., 3.];

            let result = OwnedVector::from(values.as_slice());
            assert_vec_eq(&result, values);
        }

        #[test]
        fn from_array() {
            let values = [1., 2., 3.];

            let result = OwnedVector::from(values);
            assert_vec_eq(&result, &[1., 2., 3.])
        }
    }

    mod ref_vector {
        use super::*;

        #[test]
        fn from_slice() {
            let values: &[f64] = &[1., 2., 3.];

            let result = RefVector::from(values);
            assert_vec_eq(&result, values);
        }
    }
}
