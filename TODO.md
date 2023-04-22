- [X] `UBigInt`
- [X] `BigInt`
  - `UBigInt`, is_neg: `bool`
- [X] `Ratio`
  - denom: `BigInt`, numer: `BigInt`
  - convert to/from IEEE floating point numbers

Optional: Very Big Number
- `(1, 4)` -> 2\^4
- `(2, 5)` -> 2\^(2\^5)
- `(3, 7)` -> 2\^(2\^(2\^5))

---

Eval function

`Ratio::eval("/ 3 4")` -> `Ratio { 4, 3 }`

Lisp-like syntax

---

https://doc.rust-lang.org/stable/std/fmt/index.html#traits
https://doc.rust-lang.org/stable/std/iter/trait.Sum.html
https://doc.rust-lang.org/stable/std/iter/trait.Product.html