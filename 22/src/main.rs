// Wordle
// Izzy Robbins

const RED : &str = "\u{001b}[31m";
const WHT : &str = "\u{001b}[0m";
const GRE : &str = "\u{001b}[32m";
const YEL : &str = "\u{001b}[33m";

const WORD_LEN : usize = 5;
const WORDS : [&str; WORD_LEN] = ["sator", "arepo", "tenet", "opera", "rotas"];

use std::io;
use io::Read;

fn main() {
    // Set secret word
    let mut rand_file = std::fs::File::open("/dev/random").unwrap();
    let mut buffer = [0u8; 8];
    
    rand_file.read_exact(&mut buffer).unwrap();
    let secret = usize::from_le_bytes(buffer) % WORD_LEN;
    let secret_word : &str = WORDS[secret];
    println!("{}", secret_word);

    // Get player input
    loop {
        println!("submit your word.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim();
        // compare guess to secret
        if guess.len() != 5 {
            continue;
       }
        let mut guess_chars = guess.chars();
        let mut secret_chars = secret_word.chars();
        
        for _ in 0..5 {
            let guess_char = guess_chars.nth(0).unwrap();
            let secret_char = secret_chars.nth(0).unwrap();
            if guess_char == secret_char {
                print!("{GRE}{}{WHT}", guess_char);
            } else if secret_word.contains(guess_char) {
                
                print!("{YEL}{}{WHT}", guess_char);
            } else {
              
                print!("{RED}{}{WHT}", guess_char);
            }
        }
        println!("");
    }
}
