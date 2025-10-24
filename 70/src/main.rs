// lab before ix homework
// main testing

#![allow(non_camel_case_types)]

use numerical::*;
use std::env::args;

fn main() {
    // command line argument
    let args: Vec<String> = args().collect();
    let num1 = &args[1];
    let num2 = &args[2];
    let operation = args[3].as_str();

    // preform operation over nums
    match operation {
        "ADD" => see_ix(&add_ix(&num1, &num2)),
        "SUB" => see_ix(&sub_ix(&num1, &num2)),
        "MUL" => todo!(),
        "DIV" => todo!(),
        "REM" => todo!(),
        &_ => println!("Operator not found. (ADD, SUB, MUL, DIV, REM)")
    }
}
