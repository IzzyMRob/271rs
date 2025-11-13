fn lcs(s1: &str, s2: &str) -> String {
    // for each row, each col, compare the values
    // if theyre the same, 
    let mut longest : String = "";
    let mut row = vec!();
    let mut col = vec!();

    // turn &strs into vecs so indexing is easier
    for c in s1.chars() {
        row.push(c1);
    }
    for c in s2.chars() {
        col.push(c2);
    }

    // create array to store comparios results
    let mut array : Vec<Vec<String>> = vec!();
    for i in row.len() {
        array.push(vec!());
        for _ in col.len() {
            array[i].push("");
        };
    };
    
    // comparison logic
    for ri in row.len() {
        for ci in col.len() {
            match row[ri] == col[ci] {
                True => {},
                False => {},
            }
        }
    }
    return longest
}

fn main() {
    let mut ss = std::env::args();
    let _ = &ss.next();
    dbg!(lcs(&ss.next().unwrap(), &ss.next().unwrap()));
    return;
}
