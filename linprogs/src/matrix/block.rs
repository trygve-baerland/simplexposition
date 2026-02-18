use anyhow::{Result, anyhow};

use crate::{
    matrix::{Matrix, RawMatrix},
    utils,
};

#[derive(Debug)]
pub struct BlockMatrix {
    blocks: Vec<Vec<RawMatrix>>,
    n_offsets: Vec<usize>,
    m_offsets: Vec<usize>,
}

impl BlockMatrix {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            n_offsets: Vec::new(),
            m_offsets: Vec::new(),
        }
    }

    pub fn try_push_block_row<I, Mat>(&mut self, block_row: I) -> Result<()>
    where
        I: IntoIterator<Item = Mat>,
        Mat: Into<RawMatrix>,
    {
        // Create vector of raw matrix:
        let row: Vec<RawMatrix> = block_row.into_iter().map(|b| b.into()).collect();

        // Need to verify that all matrices have the same number of rows:
        if !utils::all_same(row.iter().map(|b| b.m())) {
            return Err(anyhow!("Blocks in row where not aligned on number of rows"));
        }
        todo!()
    }

    pub fn try_push_block_column(&mut self, block_col: Vec<RawMatrix>) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_blocks() {
        let row = [[[1., 2.], [3., 4.]], [[5., 6.], [7., 8.]]];

        let mut mat = BlockMatrix::new();
        mat.try_push_block_row(row).expect("should be valid");
    }

    #[test]
    fn from_blocks_mismatched() {
        let blocks = [[[[1., 2.], [3., 4.]]]];
    }
}
