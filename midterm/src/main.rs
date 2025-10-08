use std::io::{stdin, Read, Write};
use std::fs::File;

// how to type a filename into command line:
// testing.txt
// make sure you trim input to remove newline chars!!

fn main() {
    println!("File to read:");
    let mut filename = String::new();
    stdin().read_line(&mut filename).unwrap();

    // creates a new file, erasing the ond one!! erased forever!!
    let mut out_file = File::create("out.txt").unwrap();
    let mut in_file = File::open(filename.trim()).unwrap();
    let mut long_bytes = in_file.bytes();

    // grab 3 bytes, handle wach case for the end of the file seperatly
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
                println!("Chars in match: 1:{} 2:{} 3:{} 4:{}", c_one as char, c_two as char, c_three as char, c_four as char);
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
                out_file.write(&[c_one, c_two, c_three, c_four]);
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
                out_file.write(&[c_one, c_two, c_three, c_four]);
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

// translates an integer (u8) to a base64 character
fn num_to_char(n : u8) -> u8 {
    let letters = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    let numbers = b"0123456789";
    println!("num_to_char num: {n}");
    let value = match n {
        0..52 => letters[n as usize],
        52..62 => numbers[(n-52) as usize],
        62 => b'+',
        63 => b'/',
        _ => panic!("number {} not in range", n)
    };
    println!("Value from chars: {}", value as char);
    return value;
}

