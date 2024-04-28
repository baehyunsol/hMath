mod ln;
mod pow;
mod trigo;

// e^it = cos(t) + i sin(t)
//        -> where everything begins
// e^(x + iy) = e^x (cos(y) + i sin(y))
// a^b = e^(ln(a) * b)
// ln(x + iy) = ln(r e^(i t)) = ln(r) + ln(e^(i t)) = ln(r) + i(t + 2 pi k)
//             -> it's multi-valued
// sin(x + iy) = sin(x) cosh(y) + i cos(x) sinh(y)
// cos(x + iy) = cos(x) cosh(y) − i sin(x) sinh(y)
