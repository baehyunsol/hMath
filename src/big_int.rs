mod arith;
mod comp;
mod conv;
pub mod funcs;


/*
Always in most reduced form
0 is not negative
*/
#[derive(Clone, PartialEq, Debug)]
pub struct BigInt {
    data: Vec<u32>,
    pub is_negative: bool
}


pub const BASE: u32 = 1 << 31;


impl BigInt {

    fn trim(&mut self) {

        if self.data.len() == 0 {
            return;
        }

        let mut last_index = self.data.len() - 1;

        for i in 0..last_index {
            self.data[i + 1] += self.data[i] / BASE;
            self.data[i] %= BASE;
        }

        while self.data[last_index] >= BASE {
            self.data.push(self.data[last_index] / BASE);
            self.data[last_index] %= BASE;

            last_index += 1;
        }

    }

    fn trim_zero(&mut self) {

        while self.data.len() > 0 && self.data[self.data.len() - 1] == 0 {
            self.data.pop();
        }

        if self.is_zero() {
            self.is_negative = false;
        }

    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.data.len() == 0
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn zero() -> Self {
        Self {
            data: vec![],
            is_negative: false
        }
    }

    #[inline]
    pub fn one() -> Self {
        Self {
            data: vec![1],
            is_negative: false
        }
    }

}


fn trim_u64_to_u32(mut data: Vec<u64>) -> Vec<u32> {

    if data.len() == 0 {
        return vec![];
    }

    let base_u64 = BASE as u64;
    let mut last_index = data.len() - 1;

    for i in 0..last_index {
        data[i + 1] += data[i] / base_u64;
        data[i] %= base_u64;
    }

    while data[last_index] >= base_u64 {
        data.push(data[last_index] / base_u64);
        data[last_index] %= base_u64;

        last_index += 1;
    }

    data.iter().map(|n| *n as u32).collect()
}


impl std::default::Default for BigInt {

    #[inline]
    fn default() -> BigInt {
        BigInt::zero()
    }

}


impl std::fmt::Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}