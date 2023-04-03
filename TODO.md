- [X] `UBigInt`
- [X] `BigInt`
  - `UBigInt`, is_neg: `bool`
- [ ] `Ratio`
  - denom: `BigInt`, numer: `BigInt`
  - `from_f64`: directly use IEEE754 (bitwise operation)
- [ ] `Number`
  - `BigInt` | `Ratio`
  - [ ] Auto conversion between variants
  - [ ] Convenient macro
    - `n!("1234") = Number::from_string("1234").unwrap()`
    - `n!(3.14) = Number::from_ieee754_f64(3.14)`
    - `n!(256) = Number::from_u64(256)`
    - Is it possible?

Optional: Very Big Number
- `(1, 4)` -> 2\^4
- `(2, 5)` -> 2\^(2\^5)
- `(3, 7)` -> 2\^(2\^(2\^5))

In order for it to be `sort`able, I have to impl `comp` traits in std.

---

https://doc.rust-lang.org/stable/std/fmt/index.html#traits