impl Matrix {

    // row[i] *= k
    fn ero1(&mut self, i: usize, k: &Ratio) {}

    // row[i] <-> row[j]
    fn ero2(&mut self, i: usize, j: usize) {}

    // row[i] += k * row[j]
    fn ero3(&mut self, i: usize, j: usize, k: &Ratio) {}

    pub fn inverse(&self) -> Matrix {}

}