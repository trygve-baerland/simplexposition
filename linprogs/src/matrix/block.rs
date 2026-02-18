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

        // Verify that all matrices have the same number of rows:
        if !utils::all_same(row.iter().map(|b| b.m())) {
            return Err(anyhow!("Blocks in row were not aligned on number of rows"));
        }

        // Verify that all column counts are aligned with the blocks already present.
        if !pairwise_compare(row.iter().map(|b| b.n()), self.n_offsets.iter().copied()) {
            return Err(anyhow!("Blocks in row were not aligned on columns"));
        }

        // Looks like we can add these blocks.
        self.blocks.push(row);
        Ok(())
    }

    pub fn try_push_block_column(&mut self, block_col: Vec<RawMatrix>) -> Result<()> {
        todo!()
    }
}

fn pairwise_compare(
    mut i1: impl Iterator<Item = usize>,
    mut i2: impl Iterator<Item = usize>,
) -> bool {
    // If the second iterator is empty, we say it's OK.
    let mut i2 = i2.peekable();
    if i2.peek().is_none() {
        return true;
    }

    loop {
        match (i1.next(), i2.next()) {
            (None, Some(_)) | (Some(_), None) => return false,
            (Some(e1), Some(e2)) if e1 != e2 => return false,
            (None, None) => return true,
            _ => continue,
        }
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
