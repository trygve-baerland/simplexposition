use std::ops::Mul;

use anyhow::{Result, anyhow};

pub trait Vector {
    fn n(&self) -> usize;

    fn get_unchecked(&self, index: usize) -> f64;

    fn get(&self, index: usize) -> Result<f64> {
        if index >= self.n() {
            return Err(anyhow!("invalid index {}", index));
        }
        Ok(self.get_unchecked(index))
    }
}

pub trait MutVector: Vector {
    fn get_mut_unchecked(&mut self, i: usize) -> &mut f64;

    fn get_mut(&mut self, i: usize) -> Result<&mut f64> {
        if i >= self.n() {
            return Err(anyhow!("invalid index {}", i));
        }
        Ok(self.get_mut_unchecked(i))
    }

    fn as_mut_slice(&mut self) -> &mut [f64];

    fn scale(&mut self, scale: f64) -> Result<()> {
        for i in 0..self.n() {
            let v = self.get_mut_unchecked(i);
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
        for i in 0..self.n() {
            let v = self.get_mut_unchecked(i);
            let a = to_add.get_unchecked(i);
            *v += a;
        }
        Ok(())
    }
}

impl Vector for Vec<f64> {
    fn n(&self) -> usize {
        self.len()
    }

    fn get_unchecked(&self, index: usize) -> f64 {
        self[index]
    }
}

impl MutVector for Vec<f64> {
    fn get_mut_unchecked(&mut self, i: usize) -> &mut f64 {
        &mut self[i]
    }

    fn as_mut_slice(&mut self) -> &mut [f64] {
        self.as_mut_slice()
    }
}

impl<const N: usize> Vector for &[f64; N] {
    fn n(&self) -> usize {
        self.len()
    }

    fn get_unchecked(&self, index: usize) -> f64 {
        self[index]
    }
}

impl<const N: usize> Vector for [f64; N] {
    fn n(&self) -> usize {
        self.len()
    }

    fn get_unchecked(&self, index: usize) -> f64 {
        self[index]
    }
}

impl Vector for &[f64] {
    fn n(&self) -> usize {
        self.len()
    }

    fn get_unchecked(&self, index: usize) -> f64 {
        self[index]
    }
}

impl Vector for &mut [f64] {
    fn n(&self) -> usize {
        self.len()
    }

    fn get_unchecked(&self, index: usize) -> f64 {
        self[index]
    }
}

impl MutVector for &mut [f64] {
    fn get_mut_unchecked(&mut self, i: usize) -> &mut f64 {
        &mut self[i]
    }

    fn as_mut_slice(&mut self) -> &mut [f64] {
        self
    }
}

#[derive(Debug)]
pub struct VectorView<T>(T);

impl<T: Vector> From<T> for VectorView<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Vector> Vector for VectorView<T> {
    fn n(&self) -> usize {
        self.0.n()
    }

    fn get_unchecked(&self, index: usize) -> f64 {
        self.0.get_unchecked(index)
    }
}

impl<T: MutVector> MutVector for VectorView<T> {
    fn get_mut_unchecked(&mut self, i: usize) -> &mut f64 {
        self.0.get_mut_unchecked(i)
    }

    fn as_mut_slice(&mut self) -> &mut [f64] {
        self.0.as_mut_slice()
    }
}

impl<T: Vector> Mul<f64> for VectorView<T> {
    type Output = Vec<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        (0..self.n()).map(|i| self.get_unchecked(i) * rhs).collect()
    }
}

impl<T: Vector> Mul<f64> for &VectorView<T> {
    type Output = Vec<f64>;

    fn mul(self, rhs: f64) -> Self::Output {
        (0..self.n()).map(|i| self.get_unchecked(i) * rhs).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::matrix::asserts::assert_vec_eq;

    mod vector {

        use super::*;

        #[test]
        fn from_vec() {
            let values = vec![1., 2., 3.];
            let expected = &[1., 2., 3.];

            assert_vec_eq(&values, expected);
        }

        #[test]
        fn from_slice() {
            let values = &[1., 2., 3.];
            let expected = &[1., 2., 3.];

            assert_vec_eq(&values, expected);
        }

        #[test]
        fn from_array() {
            let values = [1., 2., 3.];

            assert_vec_eq(&values, &[1., 2., 3.])
        }
    }
}
