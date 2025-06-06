use crate::MatrixError::{self, DataMismatch, InvalidMatrixSize};

#[derive(Debug, Clone, PartialEq)]
pub struct BuilderMatrix {
    rows: Option<usize>,
    cols: Option<usize>,
    data: Option<Vec<Vec<f64>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.rows == 0 || self.cols == 0 {
            return write!(f, "||");
        }
        
        for (i, row) in self.data.iter().enumerate() {
            write!(f, "|")?;

            for (j, value) in row.iter().enumerate() {
                write!(f, "{}", value)?;
                if j < self.cols - 1 {
                    write!(f, " ")?;
                }
            }

            write!(f, "|")?;

            if i < self.rows - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl Matrix {
    /// Creates a new empty matrix.
    /// # Example
    /// ```
    /// let matrix = Matrix::new();
    /// ```
    /// If you want to build a matrix use the `builder` method:
    /// ```
    /// let matrix = Matrix::builder().done().unwrap();
    /// ```
    pub fn new() -> Self {
        Self {
            rows: 1,
            cols: 1,
            data: Vec::new(),
        }
    }

    /// Creates a new instance of `BuilderMatrix` to start building a matrix.
    /// # Examples
    /// ```
    /// fn main() {
    ///    let matrix = Matrix::builder().done().unwrap();
    /// }
    /// 
    /// fn main() {
    ///     let matrix = Matrix::builder()
    ///         .rows(2)
    ///         .cols(3)
    ///         .data(vec![
    ///            vec![1.0, 2.0, 3.0],
    ///            vec![4.0, 5.0, 6.0],
    ///         ])
    ///         .done()
    ///         .unwrap();
    /// }
    /// ```
    pub fn builder() -> BuilderMatrix {
        BuilderMatrix::new()
    }

    /// Finds an element from the matrix at a specific row and column.
    /// # Example
    /// ```
    /// fn main() {
    ///     let matrix = Matrix::builder()
    ///         .rows(2)
    ///         .cols(2)
    ///         .data(vec![
    ///             vec![1.0, 2.0],
    ///             vec![3.0, 4.0],
    ///         ])
    ///         .done()
    ///         .unwrap();
    ///
    ///     assert_eq!(matrix.get(0, 1), Some(2.0));
    ///     assert_eq!(matrix.get(1, 0), Some(3.0));
    /// }
    /// ```
    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        if row < self.rows && col < self.cols {
            Some(self.data[row][col])
        } else {
            None
        }
    }

    /// Finds the location of an element.
    /// # Example
    /// ```
    /// fn main() {
    ///     let matrix = Matrix::builder()
    ///         .rows(2)
    ///         .cols(2)
    ///         .data(vec![
    ///             vec![1.0, 2.0],
    ///             vec![3.0, 4.0],
    ///         ])
    ///         .done()
    ///         .unwrap();
    ///
    ///     assert_eq!(matrix.find(2.0), Some((0, 1)));
    ///     assert_eq!(matrix.find(3.0), Some((1, 0)));
    /// }
    /// ```
    pub fn find(&self, value: f64) -> Option<(usize, usize)> {
        for (i, row) in self.data.iter().enumerate() {
            for (j, &v) in row.iter().enumerate() {
                if v == value {
                    return Some((i, j));
                }
            }
        }
        None
    }

    /// ### Function to verify if we can perform an operation with another matrix.
    /// - `is_mult` indicates if we are checking for multiplication (true) or addition (false).
    /// 
    /// If the matrixes are incompatible, it returns false.
    /// # Examples
    /// ```
    /// fn main() {
    ///     let matrix = Matrix::builder().rows(2).cols(3).data(vec![
    ///         vec![1.0, 2.0, 3.0],
    ///         vec![4.0, 5.0, 6.0],
    ///     ]);
    /// 
    ///     let matrix2 = Matrix::builder().rows(3).cols(2).data(vec![
    ///        vec![7.0, 8.0],
    ///        vec![9.0, 10.0],
    ///        vec![11.0, 12.0],
    ///     ]);
    ///
    ///     let can_multiply = matrix.verify(true, matrix2);
    ///     let can_add = matrix.verify(false, matrix2);
    /// 
    ///     assert_eq!(can_multiply, true);
    ///     assert_eq!(can_add, false);
    /// }
    /// ```
    pub fn verify(&self, is_mult: bool, other: Self) -> bool {
        match is_mult {
            true => if self.rows == other.cols { return true; },
            false => if self.rows == other.rows && self.cols == other.cols { return true; }
        }
        false
    }
}


impl BuilderMatrix {
    /// Creates a new instance of `BuilderMatrix`.
    fn new() -> Self {
        Self {
            rows: Some(1),
            cols: Some(1),
            data: Some(Vec::new()),
        }
    }
    
    /// Sets the number of rows for the matrix.
    /// - If no `data` is provided, it initializes the matrix with zeros.
    /// # Examples
    /// ```
    /// fn main() {
    ///     let matrix = Matrix::builder().rows(3).done().unwrap();
    /// }
    /// 
    /// fn main() {
    ///     let matrix = Matrix::builder()
    ///         .rows(3)
    ///         .data(vec![
    ///             vec![1.0],
    ///             vec![3.0],
    ///             vec![5.0],
    ///         ])
    ///         .done()
    ///         .unwrap();
    /// }
    /// ```
    pub fn rows(mut self, n: usize) -> Self {
        self.rows = Some(n);
        self
    }

    /// Sets the number of collums for the matrix.
    /// - If no `data` is provided, it initializes the matrix with zeros.
    /// # Examples
    /// ```
    /// fn main() {
    ///     let matrix = Matrix::builder().cols(3).done().unwrap();
    /// }
    /// 
    /// fn main() {
    ///     let matrix = Matrix::builder()
    ///         .cols(3)
    ///         .data(vec![
    ///             vec![1.0, 2.0, 3.0],
    ///         ])
    ///         .done()
    ///         .unwrap();
    /// }
    /// ```
    pub fn cols(mut self, n: usize) -> Self {
        self.cols = Some(n);
        self
    }
    
    /// Sets the data for the matrix.
    /// - If you want more than 1 element, you must provide the rows and/or columns.
    /// # Examples
    /// ```
    /// fn main() {
    ///     let matrix = Matrix::builder().data(vec![vec![8.0]]).done().unwrap();
    /// }
    /// 
    /// fn main() {
    ///     let matrix = Matrix::builder()
    ///         .rows(2)
    ///         .cols(3)
    ///         .data(vec![
    ///             vec![1.0, 2.0, 3.0],
    ///             vec![4.0, 5.0, 6.0],
    ///         ])
    ///         .done()
    ///         .unwrap();
    /// }
    /// ```
    pub fn data(mut self, data: Vec<Vec<f64>>) -> Self {
        self.data = Some(data);
        self
    }

    /// Finalizes the matrix creation process.
    /// 
    /// This method MUST be called in the end of the matrix creation chain.
    /// 
    /// It checks if the matrix has valid dimensions and data.
    /// Sometimes it will modify the matrix to ensure it will be valid.
    /// ```
    /// fn main() {
    ///     let matrix = Matrix::builder()
    ///         .rows(2)
    ///         .cols(3)
    ///         .data(vec![
    ///            vec![1.0, 2.0, 3.0],
    ///            vec![4.0, 5.0, 6.0],
    ///         ])
    ///         .done()
    ///         .unwrap();
    /// }
    /// 
    /// fn main() {
    ///     // This will return an error because the number of rows do not match the data provided.
    ///     let matrix = Matrix::builder()
    ///         .rows(2)
    ///         .cols(3)
    ///         .data(vec![
    ///             vec![1.0, 2.0, 3.0],
    ///             vec![4.0, 5.0, 6.0],
    ///             vec![7.0, 8.0, 9.0],
    ///         ])
    ///         .done()
    ///         .unwrap_err();
    /// }
    /// ```
    pub fn done(self) -> Result<Matrix, MatrixError> {
        match self {
            Self { rows, cols, .. } if rows == Some(0) || cols == Some(0) => Err(InvalidMatrixSize),
            Self { rows, cols, data } if data.clone().unwrap().is_empty() => {
                let data = vec![vec![0.0; cols.unwrap()]; rows.unwrap()];
                Ok(Matrix { rows: rows.unwrap(), cols: cols.unwrap(), data })
            }
            Self { rows, cols, data } if data.clone().unwrap().len() != rows.unwrap() || data.clone().unwrap().iter().any(|row| row.len() != cols.unwrap()) => Err(DataMismatch),
            _ => Ok(Matrix {
                rows: self.rows.unwrap(),
                cols: self.cols.unwrap(),
                data: self.data.unwrap(),
            }),
        }
    }
}