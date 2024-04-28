use crate::{Matrix, MatrixError, Ratio};

// all the numbers are `&Ratio`
/*
|a b|
|c d|

|a b c|
|d e f|
|g h i|
*/
#[macro_export]
macro_rules! determinant_hack {
    // ad - bc
    ($a: ident, $b: ident, $c: ident, $d: ident) => (
        $a.mul($d).sub(&$b.mul($c))
    );

    // -(ad - bc)
    (-, $a: ident, $b: ident, $c: ident, $d: ident) => (
        $b.mul($c).sub(&$a.mul($d))
    );

    // aei + bfg + cdh - ceg - bdi - afh
    ($a: ident, $b: ident, $c: ident, $d: ident, $e: ident, $f: ident, $g: ident, $h: ident, $i: ident) => (
        $a.mul(&$e.mul($i)).add(
            &$b.mul(&$f.mul($g))
        ).add(
            &$c.mul(&$d.mul($h))
        ).sub(
            &$c.mul(&$e.mul($g))
        ).sub(
            &$b.mul(&$d.mul($i))
        ).sub(
            &$a.mul(&$f.mul($h))
        )
    );

    // -(aei + bfg + cdh - ceg - bdi - afh)
    (-, $a: ident, $b: ident, $c: ident, $d: ident, $e: ident, $f: ident, $g: ident, $h: ident, $i: ident) => (
        $c.mul(&$e.mul($g)).add(
            &$b.mul(&$d.mul($i))
        ).add(
            &$a.mul(&$f.mul($h))
        ).sub(
            &$a.mul(&$e.mul($i))
        ).sub(
            &$b.mul(&$f.mul($g))
        ).sub(
            &$c.mul(&$d.mul($h))
        )
    );
}

impl Matrix {
    pub fn determinant(&self) -> Result<Ratio, MatrixError> {
        if !self.is_square() {
            return Err(MatrixError::NotSquare(self.cols, self.rows));
        }

        let n = self.cols;

        if n < 5 {
            if n < 3 {
                if n == 0 {
                    Err(MatrixError::EmptyMatrix)
                } else if n == 1 {
                    Ok(self.get(0, 0).clone())
                } else {
                    Ok(self.determinant_2_by_2())
                }
            } else {
                if n == 3 {
                    Ok(self.determinant_3_by_3())
                } else {
                    Ok(self.determinant_4_by_4())
                }
            }
        } else {
            let mut result = Ratio::zero();

            for j in 0..self.cols {
                result.add_mut(&self.get(0, j).mul(&self.cofactor(0, j)));
            }

            Ok(result)
        }
    }

    /// It's your responsibility to make sure that the matrix is 2 by 2.
    pub fn determinant_2_by_2(&self) -> Ratio {
        let a = self.get(0, 0);
        let b = self.get(0, 1);
        let c = self.get(1, 0);
        let d = self.get(1, 1);

        determinant_hack!(a, b, c, d)
    }

    /// It's your responsibility to make sure that the matrix is 3 by 3.
    pub fn determinant_3_by_3(&self) -> Ratio {
        let a = self.get(0, 0);
        let b = self.get(0, 1);
        let c = self.get(0, 2);
        let d = self.get(1, 0);
        let e = self.get(1, 1);
        let f = self.get(1, 2);
        let g = self.get(2, 0);
        let h = self.get(2, 1);
        let i = self.get(2, 2);

        determinant_hack!(a, b, c, d, e, f, g, h, i)
    }

    /// It's your responsibility to make sure that the matrix is 4 by 4.
    pub fn determinant_4_by_4(&self) -> Ratio {
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

        // no a0_, a_0
        let mut result = determinant_hack!(a11, a12, a13, a21, a22, a23, a31, a32, a33);
        result.mul_mut(a00);

        // no a0_, a_1
        let mut d = determinant_hack!(-, a10, a12, a13, a20, a22, a23, a30, a32, a33);
        d.mul_mut(a01);
        result.add_mut(&d);

        // no a0_, a_2
        let mut d = determinant_hack!(a10, a11, a13, a20, a21, a23, a30, a31, a33);
        d.mul_mut(a02);
        result.add_mut(&d);

        // no a0_, a_3
        let mut d = determinant_hack!(-, a10, a11, a12, a20, a21, a22, a30, a31, a32);
        d.mul_mut(a03);
        result.add_mut(&d);

        result
    }
}
