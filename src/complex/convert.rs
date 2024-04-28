use crate::{Complex, ConversionError, Ratio};

impl Complex {
    pub fn try_from_real_and_imag<R, I>(real: R, imag: I) -> Result<Self, ConversionError>
    where Ratio: TryFrom<R> + TryFrom<I>, ConversionError: From<<Ratio as TryFrom<R>>::Error> + From<<Ratio as TryFrom<I>>::Error>
    {
        Ok(Complex {
            real_: Ratio::try_from(real)?,
            imag_: Ratio::try_from(imag)?,
        })
    }

    pub fn from_real_and_imag<R, I>(real: R, imag: I) -> Self where Ratio: From<R> + From<I> {
        Complex {
            real_: Ratio::from(real),
            imag_: Ratio::from(imag),
        }
    }
}
