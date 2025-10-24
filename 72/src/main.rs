use bignum::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let a : ix = h2i_ix(&args[1].trim());
    let b : ix = h2i_ix(&args[2].trim());
    match args[3].as_str() {
        "ADD" => see_ix(&add_ix(&a, &b)),
        "SUB" => see_ix(&sub_ix(&a, &b)),
        "MUL" => todo!(),
        "DIV" => todo!(),
        "REM" => todo!(),
        &_    => println!("Operator not recognized: choose from ADD, SUB, MUL, DIV, REM"),
    }
}

fn h2i_ix(a : &str) -> ix {
    //turn input string of hexadecimal to an ix
    //parse into smaller sections, turn each into u64, push to number
    let a = &a[2..a.len()];
    let sign = true;
    let mut vals :Vec<u64> = vec!();
    let mut length = a.len();
    while length >= 16 {
        // grab last values from a, read them into hexadecimal
        let chunk = &a[length-16..length];
        vals.push(u64::from_str_radix(chunk, 16).unwrap());
        length -= 16;
    }
    if length != 0 {
        let chunk = &a[0..length];
        vals.push(u64::from_str_radix(chunk, 16).unwrap());
    }
    let number : ix = ix {
        sign,
        vals,
    };
    return number
}

fn see_ix(a : &ix) {
    for chunk in a.vals.iter().rev() {
        print!("{:x}", chunk);
    }
}
