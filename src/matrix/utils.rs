use crate::{Matrix, Ratio};

impl Matrix {
    // https://en.wikipedia.org/wiki/Minor_(linear_algebra)#First_minors
    pub(crate) fn cofactor(&self, i: usize, j: usize) -> Ratio {
        let mut c = self.minor(i, j);

        if (i + j) % 2 == 1 {
            c.neg_mut();
        }

        c
    }

    // https://en.wikipedia.org/wiki/Minor_(linear_algebra)
    pub(crate) fn minor(&self, i: usize, j: usize) -> Ratio {
        let rows = (0..i).chain((i + 1)..self.rows);
        let cols = (0..j).chain((j + 1)..self.cols);
        let mut result = Matrix::zeros(self.rows - 1, self.cols - 1);

        for (i_i, i_o) in rows.enumerate() {
            for (j_i, j_o) in cols.clone().enumerate() {
                *result.get_mut(i_i, j_i) = self.get(i_o, j_o).clone();
            }
        }

        // it doesn't check whether `self` is square or not
        //  -> this function is only used internally
        result.determinant().unwrap()
    }
}
