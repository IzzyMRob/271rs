use std::io::{stdin, Read, Write};
use std::fs::File;

// how to type a filename into command line:
// idk

fn main() {
    println!("File to read:");
    let mut filename = String::new();
    stdin().read_line(&mut filename).unwrap();

    let mut out_file = File::create("out.txt").unwrap();
    let mut in_file = File::open(filename.trim()).unwrap();
    
    let mut long_bytes = in_file.bytes();
    'main: loop {
        let f = long_bytes.next().map(|v| v.unwrap());
        let s = long_bytes.next().map(|v| v.unwrap());
        let t = long_bytes.next().map(|v| v.unwrap());
        match (f,s,t) {
            (Some(f), Some(s), Some(t)) => {
                let s_one = f >> 2;
                let s_two = (f & 0b11) << 4 | s >> 4;
                let s_three = ((s & 0b1111) << 2) | t >> 6;
                let s_four = (t << 2) >> 2;
                let (c_one, c_two, c_three, c_four) =
                    (
                        num_to_char(s_one),
                        num_to_char(s_two),
                        num_to_char(s_three),
                        num_to_char(s_four),
                     );
                out_file.write(&[c_one, c_two, c_three, c_four]);
                continue 'main;
            }
            (Some(f), Some(s), None) => {
                let s_one = f >> 2;
                let s_two = (f & 0b11) << 4 | s >> 4;
                let s_three = ((s & 0b1111) << 2);
                let s_four = b'=';
                let (c_one, c_two, c_three, c_four) =
                    (
                        num_to_char(s_one),
                        num_to_char(s_two),
                        num_to_char(s_three),
                        s_four,
                     );
                out_file.write(&[c_one, c_two, c_three, s_four]);
                break 'main
            }
            (Some(f), None, None) => {
                let s_one = f >> 2;
                let s_two = (f & 0b11) << 4;
                let s_three = b'=';
                let s_four = b'=';
                let (c_one, c_two, c_three, c_four) =
                    (
                        num_to_char(s_one),
                        num_to_char(s_two),
                        s_three,
                        s_four,
                     );
                out_file.write(&[c_one, c_two, s_three, s_four]);
                break 'main;
            }
            (None,None,None) =>{
                break 'main;
            } 
            _ => unreachable!(),
        }
     }
     write!(out_file, "\n");
}


fn num_to_char(n : u8) -> u8 {
    let letters = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    match n {
        0..52 => letters[n as usize],
        52..62 => (n - 52),
        62 => b'+',
        63 => b'/',
        _ => panic!("number {} not in range", n)
    }
}

