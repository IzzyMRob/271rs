fn pairs(mut cs:std::str::Chars, v:&mut Vec<u8>) -> Option<char> {
    return match cs.next() {
        Some('\n') => None,
        Some(n) => match pairs(cs, &mut v) {
            Some(m) => {
                println!("{}", two_hex(n,m));
                None
            },
            None => Some(n),
        },
        None => None,
    }
}


fn chars_to_vec(mut cs::std::str::Chars) -> Vec<u8> {
    let mut v = Vec::new();
    cs.next();
    cs.next();
    last(cs, v);
    if let Some(n) = pairs(cs, &mut v) {
        v.push(two_hex('0',n));
    }
    return v
}

fn main() {
    let mut guess = String::new();
    std::io::stdin().read_line(&mut guess).unwrap();
    let mut cs = guess.chars();
    cs.next();
    cs.next();
    if let Some(n) = pairs(cs, v) {
        println!("{n}");
    }
}

fn two_hex(n:char, m:char) -> u8 {
    let n = u8::from_str_radix(&n.to_string(), 16).unwrap();
    let m = u8::from_str_radix(&m.to_string(), 16).unwrap();
    return n * 16 + m;
}



