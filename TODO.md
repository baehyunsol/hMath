- [X] `UBigInt`
- [X] `BigInt`
  - `UBigInt`, is_neg: `bool`
- [X] `Ratio`
  - denom: `BigInt`, numer: `BigInt`
  - convert to/from IEEE floating point numbers

---

traits

- `add/sub/mul/div/rem`, `self/i32`, `_/mut`
- `eq/gt/lt/ne/ge/le`, `self/i32/one/zero`
- `pow`, `self/i32`
- `is_integer`, `round/floor/ceil`

you can then impl `Matrix<ThisTrait>` and `Polynomial<ThisTrait>`.

---

bitwise operations: and/or/xor/not

---

less accurate, more performant version of ratio (like fp 128)

---

Eval function

`Ratio::eval("/ 3 4")` -> `Ratio { 4, 3 }`

Lisp-like syntax

---

https://doc.rust-lang.org/stable/std/iter/trait.Sum.html
https://doc.rust-lang.org/stable/std/iter/trait.Product.html
