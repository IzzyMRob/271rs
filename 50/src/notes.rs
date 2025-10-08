// Lecture notes on floating point numbers
// why are they so terrible??
// we shall learn
//
// computers only handled integers, people need pi and for it to be constant
// thus the IEEE backed implimenting them and now it is standard.
// 
// they are not normal, do not involve magical rounding errors
// linux and windows handle them the same, by device hardware
// a double (f64) has 15 decimal places, does not mean 3 will store well
// 
// ieee float specs released 3xtimes, portable + consistant math not correct
// floating point standard guarantees:
//  symmetry of addition
//   x+y=y+x
//  additive identity
//   x+0=x
//  identity under subtraction
//   x=y -> x-y=0
//
//  ensures all numbers are unique
//  all have an opposite except 0
//  algorithms for + - * / sqrt
//
//  ieee approximates with scientific notation
//  x = 1**signbit * 2**exponent * 1.9999999
//  in binary:
//    0b 1_01010_1111000001
//    first 1 for negative
//    01010 for 10 in binary
//    (1984/2**0b01010 - 1) * 2**0b01010 for number in bits
// 32 and 64 bits are floating point standard
//  32 -> 1 sign + 8 exponent + 23 mantissa
//  64 -> 1 sign + 12 exponen + 52 mantissa
// 
// small numbers can have exact representations in floating points
//
// special numbers:
//  dividing by 0 or using infinity can be NAN
//  signed infinity - overflow protection when computer cant conceptulize
//
// machine epsilon
//  difference between 1.0 and next avalible floating point
//  smallest unit
//
//  distributed as a piecewise of linear, approx like x^3
//  less between .1-.2 than .0-.1
//
// measure precision with ulp - units in last place
// spacing between 2 values relative to l
