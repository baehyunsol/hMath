use crate::Ratio;

#[cfg(test)]
pub const RUN_ALL_TESTS: bool = false;

/// pre-calculated value of pi. It's equal to `pi_iter(7)`.
pub fn pi_const() -> Ratio {
    Ratio::from_raw(vec![0, 1996488704, 521357240, 2800251216, 760460394, 6200936], false, vec![793201965, 2280519292, 3213394952, 1450523626, 2315958499, 19480815], false)
}

/// pre-calculated value of e. It's equal to `e_iter(15)`.
pub fn e_const() -> Ratio {
    Ratio::from_raw(vec![0, 1088391135, 3969445709, 745272361], false, vec![483276161, 3835467358, 2883336425, 2025860318], false)
}

/// pre-calculated value of ln2. It's equal to `ln2_iter(11)`.
pub fn ln2_const() -> Ratio {
    Ratio::from_raw(vec![0, 0, 2136899584, 447791324, 2873720132, 2653150], false, vec![1367771821, 4197651235, 226662770, 3985341647, 3890727296, 1839023], false)
}
