use std::env::args;
use std::fs::File;
use std::io::{Read,Write};

fn main() {
    // command line argument
    let args: Vec<String> = args().collect();
    let filename = &args[1];

    // create output and input files, input into buffer as bytes
    let mut out_file = File::create("out.txt").unwrap();
    let mut in_file = File::open(filename.trim()).unwrap();
    let mut buffer = Vec::new();

    in_file.read_to_end(&mut buffer);

    // grab length as 128 for later
    let length_128 : u128 = buffer.len() as u128;
    
    // padding, add 1 then bunch of 0s
    println!("File bytes: {:?}", buffer);
    println!("length: {}",&buffer.len());
    buffer.push(128);

    'padding : loop {
        let length_bits = buffer.len() * 8;
        // corrcect length
        if length_bits % (1024 - 128) == 0 {
            break 'padding;
        }
        else {
            buffer.push(0);
        }
    }
    println!("padded: {:?}", buffer);
    println!("length: {} bits: {}", buffer.len(), buffer.len() * 8); 

    // add 128 bit representation of n to buffer
    buffer.extend(length_128.to_be_bytes());
    println!("with length: {:?}", buffer);

    // store number of chunks as a variable (N/1024)
    let chunks = (buffer.len() * 8) / 1024;
    println!("chunks: {}", chunks);

    // divide string into 16 chunks of 16 chars
    
    let chunks : Vec<Vec<u8>> = Vec::new();
    'chunks loop {
        if buffer.len() != 0 {
            chunks.push(buffer[0..128]);
            continue 'chunks
        }
        else {
            break 'chunks
        }

    }
}
