#![allow(non_camel_case_types)]
pub struct ix {
    pub sign: bool,
    pub vals: Vec<u64>,
}

pub fn add_ix(a : &ix, b : &ix) -> ix{
    // auto returns because it is last thing
    match (a.sign, b.sign) {
        (true, true) => {
            // both are positive
            // just add like normal
            let sign = true;
            let vals = add_mag(&a.vals, &b.vals);
            let sum = ix {
                sign,
                vals,
            };
            return sum
        },
        (true, false) => {
            // b is negative
            let sign = greater_mag(&a, &b);
            let vals = sub_mag(&a.vals, &b.vals);
            let sum = ix {
                sign,
                vals,
            };
            return sum
        },
        (false, true) => {
            // a is negative
            let sign = greater_mag(&a, &b);
            let vals = sub_mag(&a.vals, &b.vals);
            let sum = ix {
                sign,
                vals,
            };
            return sum
        },
        (false, false) => {
            // both are negative
            // add like normal but final is negative
            let sign = false;
            let vals = add_mag(&a.vals, &b.vals);
            let sum = ix {
                sign,
                vals,
            };
            return sum
        }
    }
}

pub fn sub_ix(a : &ix, b : &ix) -> ix{
    // adds with one sign flipped
    let b = ix {
        sign: !b.sign,
        vals: b.vals.clone(),
    };
    return add_ix(&a, &b);

}

fn greater_mag(a : &ix, b : &ix) -> bool{
    // returns true if a is larger, false if b is larger
    if a.vals.clone().len() > b.vals.clone().len() {
        return true
    }
    if b.vals.clone().len() > a.vals.clone().len() {
        return false
    }
    if a.vals[0] >= b.vals[0] {
        return true
    }
    else {
        return false
    }
}


fn add_mag(a : &Vec<u64>, b : &Vec<u64>) -> Vec<u64>{
    // carry, vec
    // add each digit from the start with the current carry
    let mut carry = 0;
    let mut sum : Vec<u64> = vec!();
    let mut length = a.len();
    for (one, two) in a.iter().zip(b.iter()) {
        // error from not pading 0s before number of it's short??
        let digit = one.wrapping_add(*two) + carry;
        carry = 0;
        // reset the carry for the new case, if we wrapped (none) carry = 1
        if one.checked_add(*two).is_none() {
            carry = 1;
        }
        sum.push(digit);
        // how many times larger is it,
        let mut count : u8 = 0;
        let mut cdigit = digit.clone();
        while cdigit >= 0x1 {
            cdigit = cdigit / 0x10;
            count += 1;
        }
        for _ in 0..(16-count) {
            sum.push(0);
        }
        length -= 1;
        // if sum.len is a.len and carry = 1, push 1
        if length == 0 && carry == 1{
            sum.push(1);
        }
    }
    return sum

}

fn sub_mag(a : &Vec<u64>, b : &Vec<u64>) -> Vec<u64> {
    // one value is negative
    // we need to handle a and b being larger
    let mut sum : Vec<u64> = vec!();
    let mut carry = 0;
    match a > b {
        true => {
            // a - b
            for (one, two) in a.iter().zip(b.iter()) {
                match one > two {
                    true => {
                        // no carying needed
                        let digit = one - two - carry;
                        sum.push(digit);
                    },
                    false => {
                        // carrying needed
                        let digit = 16 + one - two - carry;
                        carry *= 16;
                        sum.push(digit);
                    },
                }
            }
        }
        false => {
            // b - a
            for (one, two) in b.iter().zip(a.iter()) {
                match one > two {
                    true => {
                        // no carrying
                        let digit = one - two - carry;
                        sum.push(digit);
                    },
                    false => {
                        // carrying
                        let digit = 16 + one - two - carry;
                        carry *= 16;
                        sum.push(digit);
                    },
                }
            }
        }
    }
    return sum
}
