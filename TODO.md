- [X] `UBigInt`
- [X] `BigInt`
  - `UBigInt`, is_neg: `bool`
- [X] `Ratio`
  - denom: `BigInt`, numer: `BigInt`
  - convert to/from IEEE floating point numbers

---

traits

- `add/sub/mul/div/rem`, `_/i32`, `_/mut`
- `eq/gt/lt/ne/ge/le`, `_/i32/one/zero`
- `pow`, `_/i32`
- `is_integer`, `round/floor/ceil`
- `zero/one`

you can then impl `Matrix<ThisTrait>` and `Polynomial<ThisTrait>`.

---

bitwise operations: and/or/xor/not

---

Eval function

`Ratio::eval("/ 3 4")` -> `Ratio { 4, 3 }`

Lisp-like syntax
