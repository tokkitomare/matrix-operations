use crate::matrix::*;
use crate::errors::MatrixError;
pub trait Add {
    type Output;

    /// Adds two matrices together.
    ///
    /// # Arguments
    ///
    /// * `self` - The first matrix.
    /// * `other` - The second matrix.
    ///
    /// # Returns
    ///
    /// A new matrix that is the result of adding the two matrices together.
    fn add(self, other: Self) -> Result<Self::Output, MatrixError>;
}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, other: Self) -> Result<Self::Output, MatrixError> {
        if !self.verify(false, other.clone()) {
            return Err(MatrixError::DimensionMismatch);
        }

        // Build a new matrix with the same dimensions as the `self` matrix (and `other` as well).
        // It will ensure that `result` variable can be accessed in the loop below.
        let mut result = Matrix::builder().rows(self.rows).cols(self.cols).done()?;
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Ok(result)
    }
}