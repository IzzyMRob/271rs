use std::io
use std::fs::File

fn main() {
    println!("File to read:");
    let mut filename = String::new();
    stdin().read_line(&mut filename).unwrap();

    let mut out_file = File::create("out.txt").unwrap();
    let mut in_file = File::open(filename.trim()).unwrap();
    
    let long_bytes = in_file.bytes();
    'main: loop {
        let f = long_bytes.next().map(|v| v.unwrap());
        let s = long_bytes.next().map(|v| v.unwrap());
        let t = long_bytes.next().map(|v| v.unwrap());
        match () {
            (Some(f), Some(s), Some(t)) => {
                let s_one = f & 0b00111111;
                let s_two = f >> 6 | s << 2;
                let s_three = s << 4 | t >> 6;
                let s_four = t & 0b11111100;
                continue 'main;
            }
            (Some(f), Some(s), None) => {
                let s_one = f & 0b00111111;
                let s_two = f >> 6 | s << 2;
                let s_three = s >> 4;
                let s_four = '=';
                break 'main
            }
            (Some(f), None, None) => {
                let s_one = f & 0b00111111;
                let s_two = f >> 6;
                let s_three = '=';
                let s_four = '=';
                break 'main;
            }
            (None,None,None) =>{
                break 'main;
            } 
            _ => unreachable!(),
        }
     }

}


fn num_to_char(n : u8) -> char {
    letters = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
    match n {
        0..52 => letters[n as usize] as char,
        52..62 => (n - 52) as char,
        62 => '+',
        63 => '/',
        _ => panic!("number {} not in range", n)
    }
}

