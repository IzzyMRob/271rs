use std::io
use std::fs::File

fn main() {
    println!("File to read:");
    let mut filename = String::new();
    stdin().read_line(&mut filename).unwrap();

    let mut out_file = File::create("out.txt").unwrap();
    let mut in_file = File::open(filename.trim()).unwrap();
    

}


fn num_to_char(n : u8) -> char {
    latters = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
    match n {
        0..52 => letters[n as usize] as char,
        52..62 => (n - 52) as char,
        62 => '+',
        63 => '/',
        _ => panic!("number {} not in range", n)
    }
}

