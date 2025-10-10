use std::env::args;
use std::fs::File;
use std::io::{Read,Write};

fn main() {
    // command line argument
    let args: Vec<String> = args().collect();
    let filename = &args[1];
    
    // create output and input files, read input into bytes
    let mut out_file = File::create("out.txt").unwrap();
    let in_file = File::open(filename.trim()).unwrap();
    let mut long_bytes = in_file.bytes();

    write!(out_file, "<~");
    let blank = 0b00000000;
    // grab 4 bytes, turn into a 32 bit number
    'main : loop {
        let f = long_bytes.next().map(|v| v.unwrap());
        let s = long_bytes.next().map(|v| v.unwrap());
        let t = long_bytes.next().map(|v| v.unwrap());
        let l = long_bytes.next().map(|v| v.unwrap());
        match (f,s,t,l) {
            (Some(f), Some(s), Some(t), Some(l)) => {
                // concat all values together
                let combined: u32 = u32::from_be_bytes([f,s,t,l]);
                let character = num_to_asci(combined);
                out_file.write(&[character]);
                continue 'main;
            }
            (Some(f), Some(s), Some(t), None) => {
                let combined: u32 = u32::from_be_bytes([f,s,t,blank]);
                let character = num_to_asci(combined);
                out_file.write(&[character]);
                break 'main;
            }
            (Some(f), Some(s), None, None) => {
                let combined: u32 = u32::from_be_bytes([f,s,blank,blank]);
                let character = num_to_asci(combined);
                out_file.write(&[character]);
                break 'main;
            }
            (Some(f), None, None, None) => {
                let combined: u32 = u32::from_be_bytes([f,blank,blank,blank]);
                let character = num_to_asci(combined);
                out_file.write(&[character]);
                break 'main;
            }
            (None,None,None,None) => {
                break 'main;
            }
            _ => unreachable!(),
        }
    }
    write!(out_file, "~>");
}

fn num_to_asci(n : u32) -> u8 {
    let small_n : u8 = ((n % 85) + 33) as u8;
    return small_n;

}
