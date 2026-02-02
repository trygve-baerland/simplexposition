mod raw;
mod vector;

pub use raw::RawMatrix;
pub use vector::Vector;

#[cfg(test)]
mod utils {
    use crate::matrix::RawMatrix;
    const EPS: f64 = 1E-8;

    pub fn assert_mat_eq(mat: &RawMatrix, expected: &[f64]) -> () {
        assert_eq!(
            mat.values().len(),
            expected.len(),
            "actual and expected did not have same length"
        );

        for (ix, (a, e)) in mat.values().iter().zip(expected).enumerate() {
            assert!(
                (a - e).abs() < EPS,
                "matrix element at row {} and column {} was not as expected.\n\texpected: {}\n\tactual: {}",
                ix / mat.n(),
                ix % mat.n(),
                e,
                a
            )
        }
    }
}
