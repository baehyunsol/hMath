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

64/(8k+1) - 32/(8k+4) - 16/(8k+5) - 16/(8k+6) + 4/(8k+9) - 2/(8k+12) - 1/(8k+13) - 1/(8k+14)를 각 k마다 계산을 해두고 어딘가에 상수로 미리 저장. 나중에 1/256^k만 일괄적으로 곱해서 더하는 거임!!! -> 아주 빠른 pi 계산기!
-> ln(2)도 비슷한 방식으로 할 수 있지 않을까? [[math]]sum{k=1}{inf}{cfrac{1}{k sup{2}{k}}}[[/math]]이니까, 적당히 3~4개 정도씩 끊어서!