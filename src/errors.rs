use std::error::Error as StdErr;
use std::fmt::{Display, Result, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum MatrixError {
    InvalidMatrixSize,
    DimensionMismatch,
    InvalidOperation,
    DataMismatch,
}

impl StdErr for MatrixError {}

impl Display for MatrixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            MatrixError::InvalidMatrixSize => write!(f, "InvalidMatrixSize: Invalid matrix size, rows and columns must be greater than zero"),
            MatrixError::DimensionMismatch => write!(f, "DimensionMismatch: Matrix dimensions do not match"),
            MatrixError::InvalidOperation => write!(f, "InvalidOperation: Invalid operation on matrices"),
            MatrixError::DataMismatch => write!(f, "DataMismatch: Data must have the same dimensions as the matrix"),
        }
    }
    
}

impl MatrixError {
    pub fn new(error: MatrixError) -> Self {
        error
    }
}