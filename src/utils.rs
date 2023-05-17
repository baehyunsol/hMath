pub fn v32_to_v64(v32: &Vec<u32>) -> Vec<u64> {
    #[cfg(test)] assert!(v32.len() > 0);

    v32.iter().map(|n| *n as u64).collect()
}

pub fn v64_to_v32(mut v64: Vec<u64>) -> Vec<u32> {
    #[cfg(test)] assert!(v64.len() > 0);

    for i in 0..(v64.len() - 1) {

        if v64[i] >= (1 << 32) {
            v64[i + 1] += v64[i] >> 32;
            v64[i] %= 1 << 32;
        }

    }

    let v64_len = v64.len() - 1;

    if v64[v64_len] >= (1 << 32) {
        v64.push(v64[v64_len] >> 32);
        v64[v64_len] %= 1 << 32;
    }

    #[cfg(test)] assert!(v64.iter().all(|n| *n < (1 << 32)));

    v64.into_iter().map(|n| n as u32).collect()
}

pub fn remove_suffix_0(vec: &mut Vec<u32>) {

    while vec.len() > 1 && vec[vec.len() - 1] == 0 {
        vec.pop().unwrap();
    }

}

pub fn gcd_i32(mut a: i32, mut b: i32) -> i32 {
    a = a.abs();
    b = b.abs();

    while a != 0 {
        let r = b % a;
        b = a;
        a = r;
    }

    b
}

#[cfg(test)]
pub fn are_close(a: &crate::Ratio, b: &crate::Ratio, thres: f32) -> bool {

    if b.is_zero() {
        return a.is_zero();
    }

    let diff = match a.div_rat(b).abs().to_ieee754_f32() {
        Ok(n) => n,
        _ => {
            return false;
        }
    };

    1.0 - thres <= diff && diff <= 1.0 + thres
}

#[cfg(test)]
mod tests {
    use super::gcd_i32;

    #[test]
    fn gcd_i32_test() {
        let samples = vec![
            (24, 17, 1),
            (0, 0, 0),
            (0, 8, 8),
            (1728, 93, 3),
            (1048576, 84, 4),
            (3003, 343, 7)
        ];

        for (a, b, c) in samples.into_iter() {
            assert_eq!(gcd_i32(a, b), c);
            assert_eq!(gcd_i32(-a, b), c);
            assert_eq!(gcd_i32(a, -b), c);
            assert_eq!(gcd_i32(-a, -b), c);
        }

    }

} 