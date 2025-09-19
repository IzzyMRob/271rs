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
    let mut guesses: [String; 6] = [const { String::new() }; 6];

    // Set secret word
    let mut rand_file = std::fs::File::open("/dev/random").unwrap();
    let mut buffer = [0u8; 8];
    rand_file.read_exact(&mut buffer).unwrap();
    let secret = usize::from_le_bytes(buffer) % WORD_LEN;
    let secret_word : &str = WORDS[secret];
    let mut words_guessed = 0;

    // core loop
    loop {
        print_grid(&guesses, secret_word);

        // get guess
        println!("Submit your guess!");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess = guess.trim();
        
        // ensure guess is 5 chars long and add to guesses array
        if guess.len() != 5 {
            continue; // continue restarts the loop
        }
        let temp = guess.to_string();
        guesses[words_guessed] = temp;
        words_guessed += 1;
        
        // handle winning
        if guess == secret_word {
            print_grid(&guesses, secret_word);
            println!("You Won!");
            return;
        } else if words_guessed == 6 {
            print_grid(&guesses, secret_word);
            println!("Too bad...");
            return;
        }
    }
}


fn print_grid(guesses: &[String; 6], secret_word: &str) {
    let T = "┌───┬───┬───┬───┬───┐";
    let B = "└───┴───┴───┴───┴───┘";
    let E = "|   |   |   |   |   |";
    let mut rows = 0;
    println!("{}", T);

    // loop through guesses, chars and print as you go
    for g in guesses {
        if !g.is_empty() { // run through word
            let mut guess_chars = g.chars();
            let mut secret_chars = secret_word.chars();
            print!("| ");
            for i in 0..5 { // loop through characters
                let guess_char = guess_chars.nth(0).unwrap();
                let secret_char = secret_chars.nth(0).unwrap();
                if i != 4 { 
                    if guess_char == secret_char {
                        print!("{GRE}{}{WHT} | ", guess_char);
                    } else if secret_word.contains(guess_char) {
                        print!("{YEL}{}{WHT} | ", guess_char);
                    } else {
                        print!("{RED}{}{WHT} | ", guess_char);
                    }
                } else {
                    if guess_char == secret_char {
                        println!("{GRE}{}{WHT} | ", guess_char);
                    } else if  secret_word.contains(guess_char) {
                        println!("{YEL}{}{WHT} | ", guess_char);
                    } else {
                        println!("{RED}{}{WHT} | ", guess_char);
                    }
                }
            }
        } else {
            println!("{}", E);
        }
        if rows <= 4 {
            println!("├───┼───┼───┼───┼───┤");
        }
        rows = rows + 1;
        
    }
    println!("{}", B);
}
