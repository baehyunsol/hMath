use crate::{Matrix, MatrixError, determinant_hack};

impl Matrix {
    pub fn inverse(&self) -> Result<Self, MatrixError> {
        let det = self.determinant()?;

        if det.is_zero() {
            return Err(MatrixError::ZeroDeterminant);
        }

        let mut result = self.adjugate();
        result.mul_k_mut(det.reci());
        Ok(result)
    }

    fn adjugate(&self) -> Self {
        // since this function is internal,
        // it doesn't check on runtime whether `self` is square
        #[cfg(test)] assert!(self.is_square());

        let n = self.cols;

        if n < 5 {
            if n < 2 {
                self.clone()
            } else if n == 2 {
                Matrix::from_raw(
                    vec![
                        vec![self.data[1][1].clone(), self.data[0][1].neg()],
                        vec![self.data[1][0].neg(), self.data[0][0].clone()],
                    ],
                    2, 2,
                )
            } else if n == 3 {
                self.adjugate_3_by_3()
            } else {
                self.adjugate_4_by_4()
            }
        } else {
            todo!()
        }
    }

    // optimization
    // https://en.wikipedia.org/wiki/Adjugate_matrix#3_%C3%97_3_generic_matrix
    fn adjugate_3_by_3(&self) -> Self {
        let a00 = self.get(0, 0);
        let a01 = self.get(0, 1);
        let a02 = self.get(0, 2);

        let a10 = self.get(1, 0);
        let a11 = self.get(1, 1);
        let a12 = self.get(1, 2);

        let a20 = self.get(2, 0);
        let a21 = self.get(2, 1);
        let a22 = self.get(2, 2);

        Matrix::from_raw(
            vec![
                vec![determinant_hack!(a11, a12, a21, a22), determinant_hack!(-, a01, a02, a21, a22), determinant_hack!(a01, a02, a11, a12)],
                vec![determinant_hack!(-, a10, a12, a20, a22), determinant_hack!(a00, a02, a20, a22), determinant_hack!(-, a00, a02, a10, a12)],
                vec![determinant_hack!(a10, a11, a20, a21), determinant_hack!(-, a00, a01, a20, a21), determinant_hack!(a00, a01, a10, a11)],
            ],
            3, 3,
        )
    }

    fn adjugate_4_by_4(&self) -> Self {
        let a00 = self.get(0, 0);
        let a01 = self.get(0, 1);
        let a02 = self.get(0, 2);
        let a03 = self.get(0, 3);

        let a10 = self.get(1, 0);
        let a11 = self.get(1, 1);
        let a12 = self.get(1, 2);
        let a13 = self.get(1, 3);

        let a20 = self.get(2, 0);
        let a21 = self.get(2, 1);
        let a22 = self.get(2, 2);
        let a23 = self.get(2, 3);

        let a30 = self.get(3, 0);
        let a31 = self.get(3, 1);
        let a32 = self.get(3, 2);
        let a33 = self.get(3, 3);

        Matrix::from_raw(
            vec![
                vec![
                    determinant_hack!(a11, a12, a13, a21, a22, a23, a31, a32, a33),     // no a0_, a_0
                    determinant_hack!(-, a01, a02, a03, a21, a22, a23, a31, a32, a33),  // no a1_, a_0
                    determinant_hack!(a01, a02, a03, a11, a12, a13, a31, a32, a33),     // no a2_, a_0
                    determinant_hack!(-, a01, a02, a03, a11, a12, a13, a21, a22, a23),  // no a3_, a_0
                ],
                vec![
                    determinant_hack!(-, a10, a12, a13, a20, a22, a23, a30, a32, a33),  // no a0_, a_1
                    determinant_hack!(a00, a02, a03, a20, a22, a23, a30, a32, a33),     // no a1_, a_1
                    determinant_hack!(-, a00, a02, a03, a10, a12, a13, a30, a32, a33),  // no a2_, a_1
                    determinant_hack!(a00, a02, a03, a10, a12, a13, a20, a22, a23),     // no a3_, a_1
                ],
                vec![
                    determinant_hack!(a10, a11, a13, a20, a21, a23, a30, a31, a33),     // no a0_, a_2
                    determinant_hack!(-, a00, a01, a03, a20, a21, a23, a30, a31, a33),  // no a1_, a_2
                    determinant_hack!(a00, a01, a03, a10, a11, a13, a30, a31, a33),     // no a2_, a_2
                    determinant_hack!(-, a00, a01, a03, a10, a11, a13, a20, a21, a23),  // no a3_, a_2
                ],
                vec![
                    determinant_hack!(-, a10, a11, a12, a20, a21, a22, a30, a31, a32),  // no a0_, a_3
                    determinant_hack!(a00, a01, a02, a20, a21, a22, a30, a31, a32),     // no a1_, a_3
                    determinant_hack!(-, a00, a01, a02, a10, a11, a12, a30, a31, a32),  // no a2_, a_3
                    determinant_hack!(a00, a01, a02, a10, a11, a12, a20, a21, a22),     // no a3_, a_3
                ],
            ],
            4, 4,
        )
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "rand")]
    use crate::{Matrix, MatrixError};

    #[cfg(feature = "rand")]
    #[test]
    fn inv_fuzz_test() {
        for _ in 0..256 {
            for size in 2..5 {
                let mat = Matrix::generate(
                    size, size, |_, _| {
                        (rand::random::<u32>() % 4).into()
                    }
                );

                let mat_inv = match mat.inverse() {
                    Ok(m) => m,
                    Err(e) => {
                        assert_eq!(e, MatrixError::ZeroDeterminant);
                        continue;
                    },
                };

                if mat.mul(&mat_inv).unwrap() != Matrix::identity(size) {
                    panic!("{}", mat.mul(&mat_inv).unwrap().to_ratio_string());
                }
            }
        }
    }
}
