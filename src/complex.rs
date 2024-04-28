use crate::Ratio;

mod arith;
mod convert;
mod funcs;

pub struct Complex {
    real_: Ratio,
    imag_: Ratio,
}

impl Complex {
    pub fn real(&self) -> &Ratio {
        &self.real_
    }

    pub fn imaginary(&self) -> &Ratio {
        &self.imag_
    }
}
