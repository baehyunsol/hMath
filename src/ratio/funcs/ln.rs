use crate::Ratio;

/// It returns `ln(x)`. It gets more accurate as `iter` gets bigger.
pub fn ln_iter(x: &Ratio, iter: usize) -> Ratio {
    // ln(1 + x) = sum{k=1}{inf} -(-x)^k/k = x - x^2/2 + x^3/3 - x^4/4...
    todo!()
}