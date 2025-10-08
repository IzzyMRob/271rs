
pub struct f16 {
    sign : u8, //1 char
    exp : u8, //5 chars
    mantissa : u16,  // 10 chars
}

fn i32_to_f16(n:i32) {
    let bytes = n.to_ne_bytes();
    let sign = &bytes[0];
    let exp = &bytes[1..5];
    let mantissa = &bytes[6..16];

}

fn print_f16(x:f16) {
    println!("{}{}{}", x.sign, x.exp, x.mantissa)
}
