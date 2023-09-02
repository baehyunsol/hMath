use crate::Ratio;

pub use err::MatrixError;

mod det;
mod err;
mod inv;

/// It's very naively implemented, thus very slow.
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix{
    data: Vec<Vec<Ratio>>,
    cols: usize,
    rows: usize,
}

impl Matrix {
    pub fn from_vec(data: Vec<Vec<Ratio>>) -> Result<Self, MatrixError> {
        if data.is_empty() {
            return Ok(Matrix::empty());
        }

        let rows = data.len();
        let cols = data[0].len();

        for row in data.iter() {
            if row.len() != cols {
                return Err(MatrixError::InconsistetRow);
            }
        }

        Ok(Matrix { data, cols, rows })
    }

    pub fn from_vec_generic<T: Into<Ratio>>(data: Vec<Vec<T>>) -> Result<Self, MatrixError> {
        Matrix::from_vec(
            data.into_iter().map(
                |row| row.into_iter().map(
                    |n| n.into()
                ).collect::<Vec<Ratio>>()
            ).collect()
        )
    }

    pub fn from_raw(data: Vec<Vec<Ratio>>, cols: usize, rows: usize) -> Self {
        #[cfg(test)] {
            let result = Matrix::from_vec(data.clone()).unwrap();
            assert_eq!(cols, result.cols);
            assert_eq!(rows, result.rows);
        }

        Matrix { data, cols, rows }
    }

    /// result.get(i, j) = f(i, j)
    pub fn generate(cols: usize, rows: usize, f: impl Fn(usize, usize) -> Ratio) -> Self {
        Matrix {
            data: (0..rows).map(
                |i| (0..cols).map(
                    |j| f(i, j)
                ).collect::<Vec<Ratio>>()
            ).collect(),
            cols, rows,
        }
    }

    pub fn empty() -> Self {
        Matrix { data: vec![], cols: 0, rows: 0 }
    }

    pub fn zeros(cols: usize, rows: usize) -> Self {
        Matrix {
            data: vec![vec![0.into(); cols]; rows],
            cols, rows,
        }
    }

    pub fn identity(n: usize) -> Self {
        let mut result = Matrix::zeros(n, n);

        for i in 0..n {
            *result.get_mut(i, i) = Ratio::one();
        }

        result
    }

    pub fn is_square(&self) -> bool {
        self.cols == self.rows
    }

    #[inline]
    /// ith row, jth col
    pub fn get(&self, i: usize, j: usize) -> &Ratio {
        &self.data[i][j]
    }

    #[inline]
    /// ith row, jth col
    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut Ratio {
        &mut self.data[i][j]
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add(&self, other: &Matrix) -> Result<Self, MatrixError> {
        if (self.cols, self.rows) != (other.cols, other.rows) {
            return Err(MatrixError::WrongDimension {
                expected: (self.cols, self.rows),
                got: (other.cols, other.rows),
            });
        }

        let result = self.data.iter().enumerate().map(
            |(row_ind, row)| row.iter().enumerate().map(
                |(col_ind, n)| n.add_rat(other.get(row_ind, col_ind))
            ).collect::<Vec<Ratio>>()
        ).collect();

        let result = Ok(Matrix::from_raw(result, self.cols, self.rows));

        #[cfg(test)] {
            let mut k = self.clone();
            k.add_mut(other).unwrap();

            assert_eq!(result, Ok(k));
        }

        result
    }

    pub fn add_mut(&mut self, other: &Matrix) -> Result<(), MatrixError> {
        if (self.cols, self.rows) != (other.cols, other.rows) {
            return Err(MatrixError::WrongDimension {
                expected: (self.cols, self.rows),
                got: (other.cols, other.rows),
            });
        }

        for i in 0..self.rows {
            for j in 0..self.cols {
                self.get_mut(i, j).add_rat_mut(other.get(i, j));
            }
        }

        Ok(())
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn mul_k<T: Into<Ratio>>(&self, k: T) -> Self {
        let k = k.into();

        let result = self.data.iter().map(
            |row| row.iter().map(
                |n| n.mul_rat(&k)
            ).collect::<Vec<Ratio>>()
        ).collect();

        let result = Matrix::from_raw(result, self.cols, self.rows);

        #[cfg(test)] {
            let mut s = self.clone();
            s.mul_k_mut(k);

            assert_eq!(result, s);
        }

        result
    }

    pub fn mul_k_mut<T: Into<Ratio>>(&mut self, k: T) {
        let k = k.into();

        for i in 0..self.rows {
            for j in 0..self.cols {
                self.get_mut(i, j).mul_rat_mut(&k);
            }
        }
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn mul(&self, other: &Matrix) -> Result<Self, MatrixError> {
        if self.cols != other.rows {
            return Err(MatrixError::WrongDimension {
                expected: (other.cols, self.cols),
                got: (other.cols, other.rows),
            });
        }

        let mut result = Matrix::zeros(other.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result.get_mut(i, j).add_rat_mut(&self.get(i, k).mul_rat(other.get(k, j)));
                }
            }
        }

        Ok(result)
    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn transpose(&self) -> Self {
        let result = (0..self.cols).map(
            |i| (0..self.rows).map(
                |j| self.get(j, i).clone()
            ).collect::<Vec<Ratio>>()
        ).collect::<Vec<Vec<Ratio>>>();
        let result = Matrix::from_raw(result, self.rows, self.cols);

        result
    }

    pub fn transpose_mut(&mut self) {
        // can't think of any optimization
        *self = self.transpose();
    }

    pub fn to_ratio_string(&self) -> String {
        format!(
            "[{}]", self.data.iter().map(
                |row| format!(
                    "[{}]", row.iter().map(
                        |n| n.to_ratio_string()
                    ).collect::<Vec<String>>().join(", ")
                )
            ).collect::<Vec<String>>().join(", ")
        )
    }

    /// Each element is generated by `Ratio::random()`.
    #[cfg(feature = "rand")]
    pub fn random(cols: usize, rows: usize) -> Self {
        Matrix::from_raw(
            (0..rows).map(
                |_| (0..cols).map(
                    |_| Ratio::random()
                ).collect::<Vec<Ratio>>(),
            ).collect(),
            cols,
            rows,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Matrix;

    #[test]
    fn basic_test() {
        let mat0 = Matrix::from_vec_generic(vec![
            vec![7, 3, 3],
            vec![2, 9, 1],
            vec![5, 5, 3],
            vec![6, 5, 1],
        ]).unwrap();

        let mat1 = Matrix::from_vec_generic(vec![
            vec![1, 3, 6, 7],
            vec![0, 7, 2, 8],
            vec![0, 5, 1, 1],
        ]).unwrap();

        let mat2 = Matrix::from_vec_generic(vec![
            vec![3, 5, 2, 1],
            vec![0, 1, 1, 7],
            vec![1, 0, 0, 9],
        ]).unwrap();

        let mat3 = Matrix::from_vec_generic(vec![
            vec![4, 8, 8, 8],
            vec![0, 8, 3, 15],
            vec![1, 5, 1, 10],
        ]).unwrap();

        let mat4 = mat1.add(&mat1).unwrap();
        assert_eq!(mat4, mat1.mul_k(2));

        let mat5 = mat2.add(&mat2).unwrap();
        let mat5 = mat5.add(&mat2).unwrap();
        assert_eq!(mat5, mat2.mul_k(3));

        let mat6 = mat1.add(&mat2).unwrap();
        assert_eq!(mat6, mat3);

        assert_eq!(mat1.transpose().transpose(), mat1);
        assert_eq!(mat2.transpose().transpose(), mat2);
        assert_eq!(mat3.transpose().transpose(), mat3);

        let mat7 = mat0.mul(&mat1).unwrap();
        assert_eq!(mat7, Matrix::from_vec_generic(vec![
            vec![7, 57, 51, 76],
            vec![2, 74, 31, 87],
            vec![5, 65, 43, 78],
            vec![6, 58, 47, 83],
        ]).unwrap());
    }
}
