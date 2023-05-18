- [X] `UBigInt`
- [X] `BigInt`
  - `UBigInt`, is_neg: `bool`
- [X] `Ratio`
  - denom: `BigInt`, numer: `BigInt`
  - convert to/from IEEE floating point numbers

---

Optional: Very Big Number

`struct VeryBig(BigInt)` -> `n = 2^(d/4294967296)` where `n: VeryBig` and `d: BigInt`

```
let a: logged, b: logged, c: int
let a >= b

a*b -> a+b
a/b -> a-b
a+b -> a + ln(1+2^(b-a))
a-b -> a + ln(1-2^(b-a))
a^c -> a*c
```

---

Eval function

`Ratio::eval("/ 3 4")` -> `Ratio { 4, 3 }`

Lisp-like syntax

---

https://doc.rust-lang.org/stable/std/iter/trait.Sum.html
https://doc.rust-lang.org/stable/std/iter/trait.Product.html

---

Test Failures

extra_div_test
exp2_test
root_test
sin_test

something must be wrong with `UBigInt::sqrt`!!