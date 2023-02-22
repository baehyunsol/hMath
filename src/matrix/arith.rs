use crate::{Matrix, Ratio};
use std::ops::{Add, Mul};

impl Add for &Matrix {
    type Output = Matrix;

    fn add(self, other: &Matrix) -> Matrix {
        let (row, col) = self.size();
        let mut result = self.clone();

        for i in 0..row {
            for j in 0..col {
                result.elems[i][j] = &result.elems[i][j] + &other.elems[i][j];
            }
        }

        result
    }

}

impl Mul for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix {
        let (r_row, col) = self.size();
        let (row, r_col) = other.size();
        // assert_eq!(row, col);

        let mut result = Matrix::zero(r_row, r_col);

        for i in 0..r_row {

            for j in 0..r_col {

                for k in 0..col {
                    result.elems[i][j] = &result.elems[i][j] + &(&self.elems[i][k] * &other.elems[k][j]);
                }

            }

        }

        result
    }

}

impl Mul<&Ratio> for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Ratio) -> Matrix {
        let mut result = self.clone();
        let (row, col) = self.size();

        for i in 0..row {

            for j in 0..col {
                result.elems[i][j] = &result.elems[i][j] * other;
            }

        }

        result
    }

}