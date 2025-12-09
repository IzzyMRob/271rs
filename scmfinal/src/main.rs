use serde::{Deserialize, Serialize};
use serde_json::{json};
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let keyword : &str = args[1].trim();
    if keyword != "scm" {
        println!("Command '{}' not found, did you mean 'scm'?", keyword);
    };
    let blank : &str = "{'commit': {}, 'prev':`{}}";
    let mut json: serde_json::Value = serde_json::from_str(blank).unwrap();

    let command : &str = args[2].trim();
    match command {
        "init" => {
            init_func(&mut json);
        }
        "add" => {
            add_func(&mut json, args[3].trim());
        }
        "commit" => {
            let message = args[4].trim();
            json["commit"]["message"] = json!(message);
        }
        "revert" => {
            revert_func(&mut json, args[3].trim());
        }
        _ => {
            println!("Command not found, try 'init', 'add', 'commit'.");
        }
    }
}

fn init_func(json : &mut serde_json::Value) {
    json["commit"] = json!("");
    json["prev"] = json!("");
}

fn add_func(json : &mut serde_json::Value, filename : &str) {
    // get lines from file
    let file_string = fs::read_to_string(filename).unwrap();
    // store old in prev, store new in commit
    let mut lines = vec!();
    for line in file_string.lines() {
        lines.push(line.to_string());
    }
    json["prev"][filename] = json["commit"][filename].clone();
    json["commit"][filename] = json!(lines);
}

fn revert_func(json : &mut serde_json::Value, filename : &str) {
    // make prev into commit, erase prev
    json["commit"][filename] = json["prev"][filename].clone();
    json["prev"][filename] = json!("");
}
