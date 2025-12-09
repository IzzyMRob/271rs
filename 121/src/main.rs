use diff::lcs;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename_a : String = (&args[1].trim()).to_string();
    let filename_b : String = (&args[2].trim()).to_string();
    let lines_a : Vec<String> = get_lines(&filename_a);
    let lines_b : Vec<String> = get_lines(&filename_b);

    println!("Merging files: {} ({} lines), {} ({} lines)",
    filename_a,
    &lines_a.len(),
    filename_b,
    &lines_b.len());
    
    let lcs = lcs(lines_a, lines_b);
    for c in lcs.iter() {
        
    }

}

fn get_lines(filename : &String) -> Vec<String> {
    let file = File::open(filename).expect("Can't open file");
    let reader = BufReader::new(file);
    let mut lines = vec!();

    for (_, line) in reader.lines().enumerate() {
        let line = line.expect("Unable to read line");
        lines.push(line);
    }
    return lines;
}


fn sets_to_shorthand(i: usize, j: usize, ln: Vec<usize>, rn: Vec<usize>) -> String {
    
    fn side(i: usize, n: Vec<usize>) -> String {
        return match n.len() {
            // I couldn't print a `usize` so I turned them into `u64`'s and hoped for the best.
            0 => format!("{}", i as u64),
            1 => format!("{}", n[0]),
            // By the way - those vectors are backwards! Think about why they would be!
            _ => format!("{},{}", n[n.len()-1] as u64, n[0] as u64),
        }
    }
    
    let letter = match (&ln.len(), &rn.len()) {
        (0, _) => "a",
        (_, 0) => "d",
        (_, _) => "c",
    };

    return format!("{}{}{}", side(i, ln), letter, side(j, rn));
}


