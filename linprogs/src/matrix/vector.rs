use std::slice::IterMut;

use anyhow::Result;

pub trait Vector<'a> {
    fn values(&'a self) -> &'a [f64];

    fn values_mut(&'a mut self) -> &'a mut [f64];

    fn n(&self) -> usize;
}

#[derive(Debug)]
pub struct SliceMutRef<'a> {
    values: &'a mut [f64],
}

impl<'a> SliceMutRef<'a> {
    pub fn iter_mut(&mut self) -> IterMut<'_, f64> {
        self.values.iter_mut()
    }

    pub fn scale(&'a mut self, scale: f64) -> Result<()> {
        for v in self.values_mut() {
            *v *= scale
        }
        Ok(())
    }
}

impl<'a> From<&'a mut [f64]> for SliceMutRef<'a> {
    fn from(value: &'a mut [f64]) -> Self {
        Self { values: value }
    }
}

impl<'a> Vector<'a> for SliceMutRef<'a> {
    fn values(&'a self) -> &'a [f64] {
        self.values
    }

    fn values_mut(&'a mut self) -> &'a mut [f64] {
        self.values
    }

    fn n(&self) -> usize {
        self.values.len()
    }
}
