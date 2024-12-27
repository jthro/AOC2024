fn main() {
    let input: Vec<Vec<char>> = std::fs::read_to_string("./input.txt").unwrap().split_terminator('\n').map(|l| l.to_owned().chars().collect()).collect();
    let mut copy: Vec<Vec<char>> = input.clone();

    let start_row = input.iter().position(|r| r.contains(&'^')).unwrap() as isize;
    let start_col = input[start_row as usize].iter().position(|c| c == &'^').unwrap() as isize;

    let mut dir: (isize, isize) = (-1, 0);

    let mut row = start_row.clone();
    let mut col = start_col.clone();

    while row >= 0 && row < input[0].len() as isize && col >= 0 && col < input.len() as isize {
        if input[row as usize][col as usize] == '#' {
            row -= dir.0;
            col -= dir.1;


            dir = rotate_clockwise(dir);

            row += dir.0;
            col += dir.1;

            continue;
        }

        copy[row as usize][col as usize] = 'X';
        row += dir.0;
        col += dir.1;
    }

    let result = copy.iter().fold(0, |acc, x| acc + x.iter().filter(|c| c == &&'X').count());
    dbg!(result);

    // part 2

    let mut result = 0;
    let mut store_directions: Vec<Vec<Vec<(isize, isize)>>> = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        store_directions.push(Vec::with_capacity(input[0].len()));
        for _ in 0..input[0].len() {
            store_directions[i].push(Vec::new());
        }
    }

    dbg!(store_directions.len());


    for (i, r) in copy.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if c == &'X' {
                println!("New Obstacle");
                let mut curr_copy = input.clone();
                curr_copy[i][j] = '#';

                let mut store_directions_copy = store_directions.clone();
                let mut row = start_row.clone();
                let mut col = start_col.clone();
                let mut dir: (isize, isize) = (-1, 0);

                while row >= 0 && row < input[0].len() as isize && col >= 0 && col < input.len() as isize {
                    if curr_copy[row as usize][col as usize] == '#' {
                        row -= dir.0;
                        col -= dir.1;


                        dir = rotate_clockwise(dir);

                        row += dir.0;
                        col += dir.1;

                        continue;
                    }

                    if store_directions_copy[row as usize][col as usize].contains(&(dir.0, dir.1)) {
                        result += 1;
                        break;
                    }

                    store_directions_copy[row as usize][col as usize].push(dir.clone());

                    row += dir.0;
                    col += dir.1;
                }
            }
        }
    }

    dbg!(result);
}

fn rotate_clockwise(vector: (isize, isize)) -> (isize, isize) {
    match vector {
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => (0, 1),
    }
}
