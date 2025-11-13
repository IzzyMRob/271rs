fn lcs(s1: &str, s2: &str) -> String {
    // for each row, each col, compare the values
    // if theyre the same, append char to upper left sequence
    // if different, set to longest of left and up
    let mut longest : String = "".to_string();
    let mut row = vec!();
    let mut col = vec!();

    // turn &strs into vecs so indexing is easier
    row.push(' ');
    col.push(' ');
    for c in s1.chars() {
        row.push(c);
    }
    for c in s2.chars() {
        col.push(c);
    }

    // create array to store comparios results
    let mut array : Vec<Vec<String>> = vec!();
    for i in 0..row.clone().len() {
        array.push(vec!());
        for _ in 0..col.clone().len() {
            array[i].push("".to_string());
        };
    };

    // comparison logic
    for ri in 0..row.clone().len() {
        for ci in 0..col.clone().len() {
            match &row[ri] == &col[ci] {
                
                // they are the same character
                true => {
                    // append char to up+left value
                    if ri > 0 && ci > 0 {
                        array[ri][ci] = array[ri-1][ci-1].clone();
                        array[ri][ci].push(row[ri]);
                    }
                },
                // they are not the same character
                
                false => {
                    // both greater, means we check for longer
                    if ri > 0 && ci > 0{
                        if array[ri-1][ci].len() > array[ri][ci-1].len() {
                            array[ri][ci] = array[ri-1][ci].clone();
                        } else {
                            array[ri][ci] = array[ri][ci-1].clone();
                        }
                    // row index greater, must be r-1
                    } else if ri > 0 {
                        array[ri][ci] = array[ri-1][ci].clone();
                    // col index greater, must be c-1
                    } else if ci > 0 {
                        array[ri][ci] = array[ri][ci-1].clone();
                    }   
                },
            } 
            // if current box is the longest, store in longest
            if array[ri][ci].len() > longest.len() {
                longest = array[ri][ci].clone();
            };
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
