use core::str;
use std::fs;

fn main() {
    let contents: String = fs::read_to_string("./input.txt").unwrap();
    let mut rows: Vec<String> = contents.split_terminator('\n').map(str::to_string).collect();


    let resultstr: Vec<&str> = rows.iter().map(|s| s.as_str()).collect();
    let mut resultu8: Vec<Vec<u8>> = Vec::new();

    for s in resultstr {
        unsafe {
            let bytes = std::mem::transmute::<&str, &[u8]>(s);
            resultu8.push(bytes.to_vec());
        }
    }

    let mut result: usize = 0;

    for w in resultu8.windows(4) {
        for (i, _) in w[0].iter().enumerate() {
            if i < w[0].len() - 3 {
                let diag = String::from_utf8(vec!(w[0][i], w[1][i+1], w[2][i+2], w[3][i+3])).unwrap();
                if diag == "XMAS" || diag == "SAMX" {
                    result += 1;
                }

                let diag = String::from_utf8(vec!(w[3][i], w[2][i+1], w[1][i+2], w[0][i+3])).unwrap();
                if diag == "XMAS" || diag == "SAMX" {
                    result += 1;
                }

            }

            let vertical = String::from_utf8(vec!(w[0][i], w[1][i], w[2][i], w[3][i])).unwrap();
            if vertical == "XMAS" || vertical == "SAMX" {
                result += 1;
            }
        }
    }

    for row in resultu8.iter() {
        for w in row.windows(4) {
            let horizontal = String::from_utf8(w.to_vec()).unwrap();
            if horizontal == "XMAS" || horizontal == "SAMX" {
                result += 1;
            }
        }
    }

    dbg!(result);

    // Part 2

    rows = contents.split_terminator('\n').map(str::to_string).collect();
    result = 0;

    let resultstr: Vec<&str> = rows.iter().map(|s| s.as_str()).collect();
    let mut resultu8: Vec<Vec<u8>> = Vec::new();

    for s in resultstr {
        unsafe {
            let bytes = std::mem::transmute::<&str, &[u8]>(s);
            resultu8.push(bytes.to_vec());
        }
    }

    for (i, row) in resultu8.windows(3).enumerate() {
        if i >= rows.len() - 2 {
            break;
        }

        for (j, _) in row[0].iter().enumerate() {
            if j >= row[0].len() - 2 {
                break;
            }
            let diag1: String = String::from_utf8(vec!(row[0][j], row[1][j+1], row[2][j+2])).unwrap();
            let diag2: String = String::from_utf8(vec!(row[0][j + 2], row[1][j+1], row[2][j])).unwrap();

            if (diag1 == "MAS" || diag1 == "SAM") && (diag2 == "MAS" || diag2 == "SAM") {
                result += 1;
            }
        }
    }

    dbg!(result);

}
