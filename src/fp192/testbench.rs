use crate::F192;

pub fn assert_very_close(a: F192, b: F192) {
    assert_close_as(a, b, 32);
}

pub fn assert_f64_close(a: F192, b: F192) {
    // f64 mantissa 52 bits
    // f192 mantissa 127 bits
    // 2 more bits for tolerance
    assert_close_as(a, b, 1 << 77);
}

fn assert_close_as(a: F192, b: F192, limit: u128) {
    match a.exp.abs_diff(b.exp) {
        0 => match a.digits.abs_diff(b.digits) {
            x if x < limit => {},
            _ => panic_two_f192s(a, b),
        }
        1 => panic_two_f192s(a, b),  // extra logic?
        _ => panic_two_f192s(a, b),
    }
}

fn panic_two_f192s(a: F192, b: F192) {
    panic!("a: {a} {a:?}\nb: {b} {b:?}");
}
