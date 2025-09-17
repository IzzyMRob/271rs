//Guessing Game

const RED : &str = "\u{001b}[31m";
const WHT : &str = "\u{001b}[0m";
const GRE : &str = "\u{001b}[032m";
const YEL : &str = "\u{001b}[033m";

// use the input/output library from standard
use std::io;
use std::cmp::Ordering;
// Random crate
use rand::Rng;

fn main() {
    // create and print secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);


    loop {
        // create a new empty mutable variable to store the guess
        println!("Please input your guess.");
        let mut guess = String::new();

        // get input from terminal and read it, making guess a reference to it
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // trim removes whitespace, parse changes type, match to ok or error case
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {guess}");

        // cmp - compare, type enum with 3x variants
        // match expression: arms with a pattern to match against and code to run if matched
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}    
