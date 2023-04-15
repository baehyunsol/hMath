use crate::consts::U64_32;

pub fn v32_to_v64(v32: &Vec<u32>) -> Vec<u64> {
    #[cfg(test)] assert!(v32.len() > 0);

    v32.iter().map(|n| *n as u64).collect()
}

pub fn v64_to_v32(mut v64: Vec<u64>) -> Vec<u32> {
    #[cfg(test)] assert!(v64.len() > 0);

    for i in 0..(v64.len() - 1) {

        if v64[i] >= U64_32 {
            v64[i + 1] += v64[i] / U64_32;
            v64[i] %= U64_32;
        }

    }

    let v64_len = v64.len() - 1;

    if v64[v64_len] >= U64_32 {
        v64.push(v64[v64_len] / U64_32);
        v64[v64_len] %= U64_32;
    }

    #[cfg(test)] assert!(v64.iter().all(|n| *n < U64_32));

    v64.into_iter().map(|n| n as u32).collect()
}

pub fn remove_suffix_0(vec: &mut Vec<u32>) {

    while vec.len() > 1 && vec[vec.len() - 1] == 0 {
        vec.pop().unwrap();
    }

}