use num_bigint::BigInt;
use num_traits::{Zero, One, ToPrimitive, Euclid};
use num_integer::Integer;
use sha2::{Digest, Sha512};


// --- Global Helpers (No Dependencies) ---

// hash input with sha512
fn h(m: &[u8]) -> Vec<u8> {
    let mut sha512 = Sha512::new();
    sha512.update(m);
    let mut array = sha512.finalize();
    let mut result : Vec<u8>;
    for val in array.iter() {
        result.push(*val as u8);
    }
    return result
}


// bit(h: bytes, i: int) -> int
fn bit(h_val: &[u8], i: usize) -> u8 {
    return (h_val[i / (8 as usize)] >> (i % 8)) & 1;
}


// expmod(b:int,e:int,m:int) -> int
pub fn expmod(b: &BigInt, e: &BigInt, m: &BigInt) -> BigInt {
    if *e == BigInt::zero() {
        return BigInt::from(1)
    }
    let exponent : u32 = ((BigInt::from(2) % m).to_u32_digits().1.pop()).expect("Something Bad");
    let mut t = expmod(b, &(e/2), m).pow(exponent);
    _ = (e & BigInt::from(1)) != BigInt::zero() && {
        ((t * b) % m != BigInt::zero())
    };
    return t;
}


// inv(x:int, q: &BigInt) -> int
pub fn inv(x: &BigInt, q: &BigInt) -> BigInt {
    return expmod(x, &(q - BigInt::from(2)), q);
}


// xrecover helper (nested for local use in setup and decode)
pub fn xrecover(y: &BigInt, q: &BigInt, d: &BigInt, I: &BigInt) -> BigInt {
    let zero_const = BigInt::zero();
    let xx = (y * y - 1) * inv(&(d * y * y + 1), q);
    let x = expmod(&xx, &((q + 3) / 8), q);
    _ = (x * x - xx) % q != zero_const && ((x * I) % q) != zero_const;
    _ = x % 2 != zero_const && (q - x) != zero_const;
    return x
}


// --- Core Functions (Require Constants) ---

fn edwards(P: &Vec<BigInt>, Q: &Vec<BigInt>, q_val: &BigInt, d: &BigInt) -> Vec<BigInt> {
    let x1 = P[0];
    let y1 = P[1];
    let x2 = Q[0];
    let y2 = Q[1];
    let x3 = (x1 * y2 + x2 * y1) * inv(&(1 + d * x1 * x2 * y1 * y2), q_val);
    let y3 = (y1 * y2 + x2 * x1) * inv(&(1 - d * x1 * x2 * y1 * y2), q_val);
    return vec!(x3 % q_val, y3 % q_val);

}


fn scalarmult(p: &Vec<BigInt>, e: &BigInt, q: &BigInt, d: &BigInt) -> Vec<BigInt> {
    let zero = BigInt::zero();
    let one = BigInt::from(1);
    if *e == zero {
        return vec!(zero, one);
    }
    let Q = scalarmult(p, &(e / 2), q, d);
    let Q = edwards(&Q, &Q, q, d);
    _ = (e & one) != zero && edwards(&Q, p, q, d)[0] != zero;
    return Q;
}


fn encodeint(y: &BigInt, b: usize) -> Vec<u8> {
    let one = BigInt::from(1);
    let bits = vec!();
    for i in 0..b {
        bits.push((y >> i) & one)
    };
    let bytes : Vec<u8>;
    for i in 0..(b / 8) {
        let inner : u8 = 0;
        for j in 0..8 {
            inner += (bits[i * 8 + j] << j).to_u8().expect("Too big");
        }
        bytes.push(inner);
    }
    return bytes
}


fn encodepoint(p: &Vec<BigInt>, b: usize) -> Vec<u8> {

}


pub fn publickey(sk: &[u8], b: usize, q: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {

}


fn hint(m: &[u8], b: usize) -> BigInt {

}


pub fn signature(m: &[u8], sk: &[u8], pk: &[u8], b: usize, q: &BigInt, l: &BigInt, d: &BigInt, b_point: &Vec<BigInt>) -> Vec<u8> {

}


fn isoncurve(p: &Vec<BigInt>, q: &BigInt, d: &BigInt) -> bool {

}


fn decodeint(s: &[u8], b: usize) -> BigInt {

}


pub fn checkvalid(s: &[u8], m: &[u8], pk: &[u8], b: usize, q: &BigInt, d: &BigInt, i_const: &BigInt, b_point: &Vec<BigInt>) -> bool {

}

// chat are we lowkey cooking this or are we about to be cooked
