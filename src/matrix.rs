use crate::Ratio;

mod arith;

#[derive(Clone)]
pub struct Matrix {
    pub elems: Vec<Vec<Ratio>>
}

impl Matrix {

    pub fn zero(row: usize, col: usize) -> Self {
        Matrix {
            elems: vec![vec![Ratio::zero(); col]; row]
        }
    }

    pub fn id(size: usize) -> Self {
        let mut result = Matrix::zero(size, size);

        for i in 0..size {
            result.elems[i][i] = Ratio::one();
        }

        result
    }

    pub fn size(&self) -> (usize, usize) {  // (rows, cols)

        if self.elems.len() == 0 {
            (0, 0)
        }

        else {
            (self.elems.len(), self.elems[0].len())
        }

    }

}