use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let ingit : &str = args[1].trim();
    if ingit != "scm" {
        println!("Command '{}' not found, did you mean 'scm'?", ingit);
    };
    let blank : &str = "{'latest': {}, 'commit': {}, 'prev':`{}}";


    let command : &str = args[2].trim();
    match command {
        "init" => {
            init_func(blank);
        }
        "add" => {
            let filename : &str = args[3].trim();
            println!("{}", filename);
            add_func(filename);
        }
        "commit" => {}
        _ => unreachable!()
    }
}

fn init_func(blank : &str) {
    let mut json: serde_json::Value = serde_json::from_str(blank).unwrap();
    json["commit"] = json!("");
    json["latest"] = json!("");
    json["prev"] = json!("");
}

fn add_func(filename : &str) {
    let file = File::open(filename);
    let reader = BufReader::new(file).lines();
    //reader.lines().collect::<Result<Vec<String>, io::Error>>()

}


