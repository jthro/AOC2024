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

    for row in resultu8.windows(4) {

        for (j, _) in row[0].iter().enumerate() {

            if j < row[0].len() - 3 {
                let rowmas: String = String::from_utf8(vec!(row[0][j], row[0][j+1], row[0][j+2], row[0][j+3])).unwrap();
                if rowmas == "XMAS" || rowmas == "SAMX" {
                    result += 1;
                }

                let diag1: String = String::from_utf8(vec!(row[0][j], row[1][j+1], row[2][j+2], row[3][j+3])).unwrap();
                let diag2: String = String::from_utf8(vec!(row[0][j + 3], row[1][j+2], row[2][j+1], row[3][j])).unwrap();

                if diag1 == "XMAS" || diag1 == "SAMX" {
                    result += 1;
                }
                if diag2 == "XMAS" || diag2 == "SAMX" {
                    result += 1;
                }
            }

            let colmas: String = String::from_utf8(vec!(row[0][j], row[1][j], row[2][j], row[3][j])).unwrap();

            if colmas == "XMAS" || colmas == "SAMX" {
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
