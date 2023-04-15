use crate::UBigInt;
use crate::consts::U64_32;
use crate::utils::{v32_to_v64, v64_to_v32};

impl UBigInt {

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add_ubi(&self, other: &UBigInt) -> Self {
        let mut result: Vec<u64> = (0..self.len().min(other.len())).map(
            |i| self.0[i] as u64 + other.0[i] as u64
        ).collect();

        if self.len() > other.len() {

            for i in other.len()..self.len() {
                result.push(self.0[i] as u64);
            }

        }

        else {

            for i in self.len()..other.len() {
                result.push(other.0[i] as u64);
            }

        }

        let result = UBigInt::from_raw(v64_to_v32(result));

        #[cfg(test)] {
            let mut t = self.clone();
            t.add_ubi_mut(other);
            assert_eq!(t, result);
            assert!(result.is_valid());
        }

        result
    }

    pub fn add_ubi_mut(&mut self, other: &UBigInt) {
        let mut carry = false;

        for i in 0..self.len().min(other.len()) {

            match self.0[i].checked_add(other.0[i]) {
                Some(n) => if carry {
                    match n.checked_add(1) {
                        Some(n) => {
                            self.0[i] = n;
                            carry = false;
                        }
                        _ => {
                            self.0[i] = ((self.0[i] as u64 + other.0[i] as u64 + 1) % U64_32) as u32;
                        }
                    }
                } else {
                    self.0[i] = n;
                }
                _ => if carry {
                    self.0[i] = ((self.0[i] as u64 + other.0[i] as u64 + 1) % U64_32) as u32;
                } else {
                    self.0[i] = ((self.0[i] as u64 + other.0[i] as u64) % U64_32) as u32;
                    carry = true;
                }
            }

        }

        if other.len() > self.len() {

            for i in self.len()..other.len() {

                // TODO: branches are expensive
                if carry {

                    match other.0[i].checked_add(1) {
                        Some(n) => {
                            self.0.push(n);
                            carry = false;
                        }
                        _ => {
                            self.0.push(0);
                        }
                    }

                }

                else {
                    self.0.push(other.0[i]);
                }

            }

        }

        else if self.len() > other.len() && carry {

            for i in other.len()..self.len() {

                if self.0[i] == u32::MAX {
                    self.0[i] = 0;
                }

                else {
                    self.0[i] += 1;
                    carry = false;
                    break;
                }

            }

        }

        if carry {
            self.0.push(1);
        }

    }

    #[must_use = "method returns a new number and does not mutate the original value"]
    pub fn add_u32(&self, other: u32) -> Self {
        let mut result = self.clone();
        result.add_u32_mut(other);

        #[cfg(test)] {
            let mut s2 = self.clone();
            s2.add_ubi_mut(&UBigInt::from_u32(other));
            let t = self.add_ubi(&UBigInt::from_u32(other));
            assert_eq!(t, result);
            assert_eq!(t, s2);
            assert!(result.is_valid());
        }

        result
    }

    pub fn add_u32_mut(&mut self, other: u32) {

        match self.0[0].checked_add(other) {
            Some(n) => {
                self.0[0] = n;
            }
            None => {
                let mut self_data = v32_to_v64(&self.0);
                self_data[0] += other as u64;

                self.0 = v64_to_v32(self_data);
            }
        }

    }

}