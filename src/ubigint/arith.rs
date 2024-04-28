use crate::UBigInt;

mod add;
mod div;
mod mul;
mod pow;
mod rem;
mod sub;

impl std::iter::Sum for UBigInt {
    fn sum<I: Iterator<Item = UBigInt>>(mut iter: I) -> Self {
        let mut result = UBigInt::zero();

        while let Some(n) = iter.next() {
            result.add_ubi_mut(&n);
        }

        result
    }
}

impl std::iter::Product for UBigInt {
    fn product<I: Iterator<Item = UBigInt>>(mut iter: I) -> Self {
        let mut result = UBigInt::one();

        while let Some(n) = iter.next() {
            result.mul_ubi_mut(&n);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::UBigInt;

    #[test]
    fn sum_prod_test() {
        let v1 = (1..21).map(|n| UBigInt::from_u32(n)).collect::<Vec<UBigInt>>();

        //assert_eq!(v1.iter().sum::<UBigInt>(), UBigInt::from_u32(210));
        assert_eq!(v1.into_iter().sum::<UBigInt>(), UBigInt::from_u32(210));
    }
}
