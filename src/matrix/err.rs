#[derive(Debug, PartialEq)]
pub enum MatrixError {
    WrongDimension {
        expected: (usize, usize),
        got: (usize, usize),
    },
    InconsistetRow,

    /// (cols, rows)
    NotSquare(usize, usize),

    /// an unempty matrix is expected, but got an empty matrix
    EmptyMatrix,

    /// a non-zero determinant is expected, but got 0
    ZeroDeterminant,
}
