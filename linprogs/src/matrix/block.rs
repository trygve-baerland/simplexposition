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

    pub fn as_raw(&self) -> RawMatrix {
        // Let's get the total number of elements
        let n = self.n_offsets.iter().sum();
        let m = self.m_offsets.iter().sum();
        let mut values = Vec::with_capacity(n * m);

        for (ri, row) in self.blocks.iter().enumerate() {
            for i in 0..self.m_offsets[ri] {
                for block in row {
                    for j in 0..block.n() {
                        values.push(block.get_unchecked(i, j));
                    }
                }
            }
        }
        RawMatrix::try_new(values, n, m).expect("Panic if something is wrong here")
    }

    pub fn try_push_block_row(
        &mut self,
        block_row: impl IntoIterator<Item = RawMatrix>,
    ) -> Result<()> {
        // Create vector of raw matrix:
        let row: Vec<RawMatrix> = block_row.into_iter().collect();

        // Verify that all matrices have the same number of rows:
        if !utils::all_same(row.iter().map(|b| b.m())) {
            return Err(anyhow!("Blocks in row were not aligned on number of rows"));
        }

        // Verify that all column counts are aligned with the blocks already present.
        if !pairwise_compare(row.iter().map(|b| b.n()), self.n_offsets.iter().copied()) {
            return Err(anyhow!("Blocks in row were not aligned on columns"));
        }

        // Looks like we can add these blocks.
        self.m_offsets.push(row.get(0).map_or(0, |b| b.m()));

        // If this is the first row being added, we need to added column offsets as well
        if self.n_offsets.is_empty() {
            self.n_offsets = row.iter().map(|b| b.n()).collect();
        }

        self.blocks.push(row);
        Ok(())
    }

    pub fn try_push_block_column(
        &mut self,
        block_col: impl IntoIterator<Item = RawMatrix>,
    ) -> Result<()> {
        // Create vector of raw matrix:
        let col: Vec<RawMatrix> = block_col.into_iter().collect();

        // Verify that all matrices have the same number of rows:
        if !utils::all_same(col.iter().map(|b| b.n())) {
            return Err(anyhow!(
                "Blocks in row were not aligned on number of columns"
            ));
        }

        // Verify that all row counts are aligned with the blocks already present.
        if !pairwise_compare(col.iter().map(|b| b.m()), self.m_offsets.iter().copied()) {
            return Err(anyhow!("Blocks in row were not aligned on rows"));
        }

        // Looks like we can add these blocks.
        self.n_offsets.push(col.get(0).map_or(0, |b| b.n()));

        // If this is the first column, we also need to add the row offsets
        if self.m_offsets.is_empty() {
            self.m_offsets = col.iter().map(|b| b.m()).collect();
            // Also need to initialize inner block vector
            for _ in 0..col.len() {
                self.blocks.push(Vec::new());
            }
        }

        for (row, el) in self.blocks.iter_mut().zip(col) {
            row.push(el)
        }
        Ok(())
    }
}

fn pairwise_compare(mut i1: impl Iterator<Item = usize>, i2: impl Iterator<Item = usize>) -> bool {
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
    use crate::matrix::asserts::assert_mat_eq;

    use super::*;

    #[test]
    fn from_row_blocks() {
        let row: [RawMatrix; _] = [[[1., 2.], [3., 4.]].into(), [[5., 6.], [7., 8.]].into()];

        let mut mat = BlockMatrix::new();
        mat.try_push_block_row(row).expect("should be valid");

        assert_mat_eq(&mat.as_raw(), &[1., 2., 5., 6., 3., 4., 7., 8.])
    }

    #[test]
    fn multiple_rows() {
        let r1: [RawMatrix; _] = [[[1., 2.]].into(), [[3.]].into()];
        let r2: [RawMatrix; _] = [[[4., 5.]].into(), [[6.]].into()];

        let mut mat = BlockMatrix::new();
        mat.try_push_block_row(r1).expect("should be valid");
        mat.try_push_block_row(r2).expect("should also be valid");

        assert_mat_eq(&mat.as_raw(), &[1., 2., 3., 4., 5., 6.]);
    }

    #[test]
    fn mismatched_rows() {
        let r1: [RawMatrix; _] = [[[1., 2.]].into(), [[3.]].into()];
        let r2: [RawMatrix; _] = [[[4., 5.]].into(), [[6., 7.]].into()];

        let mut mat = BlockMatrix::new();
        mat.try_push_block_row(r1).expect("should be valid");
        let err = mat.try_push_block_row(r2);
        assert!(err.is_err());
    }

    #[test]
    fn from_col_blocks() {
        let col: [RawMatrix; _] = [[[1., 2.], [3., 4.]].into(), [[5., 6.], [7., 8.]].into()];

        let mut mat = BlockMatrix::new();
        mat.try_push_block_column(col).expect("should be valid");

        let raw = mat.as_raw();
        assert_eq!(raw.n(), 2);
        assert_eq!(raw.m(), 4);
        assert_mat_eq(&mat.as_raw(), &[1., 2., 3., 4., 5., 6., 7., 8.])
    }

    #[test]
    fn multiple_cols() {
        let c1: [RawMatrix; _] = [[[1., 2.]].into(), [[3., 4.]].into()];
        let c2: [RawMatrix; _] = [[[5., 6.]].into(), [[7., 8.]].into()];

        let mut mat = BlockMatrix::new();
        mat.try_push_block_column(c1).expect("should be valid");
        mat.try_push_block_column(c2).expect("should be valid");

        assert_mat_eq(&mat.as_raw(), &[1., 2., 5., 6., 3., 4., 7., 8.]);
    }

    #[test]
    fn mismatched_cols() {
        let c1: [RawMatrix; _] = [[[1., 2.]].into(), [[3., 4.]].into()];
        let c2: [RawMatrix; _] = [[[5., 6.]].into(), [[7.]].into()];

        let mut mat = BlockMatrix::new();
        mat.try_push_block_column(c1).expect("should be valid");
        assert!(mat.try_push_block_column(c2).is_err());
    }

    #[test]
    fn from_blocks_mismatched() {
        let blocks = [[[[1., 2.], [3., 4.]]]];
    }
}
