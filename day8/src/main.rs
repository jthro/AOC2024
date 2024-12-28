fn main() {
    let input: Vec<Vec<char>> = std::fs::read_to_string("./input.txt").unwrap().split_terminator("\n").map(|l| l.to_owned().chars().collect()).collect();
    let mut output = input.clone();

    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '.' || *c == '#' {
                continue;
            }
            for (yi, rowi) in input.iter().enumerate() {
                for (xi, ci) in rowi.iter().enumerate() {
                    if xi == x && yi == y {
                        continue;
                    }
                    if c == ci {
                        let dx = x as isize - xi as isize;
                        let dy = y as isize - yi as isize;

                        let poss_x = xi as isize - dx;
                        let poss_y = yi as isize - dy;


                        if poss_x >= 0 && poss_x < input[0].len() as isize && poss_y >= 0 && poss_y < input.len() as isize && output[poss_y as usize][poss_x as usize] != '#' {
                            output[poss_y as usize][poss_x as usize] = '#';
                        }
                    }
                }
            }
        }
    }

    dbg!(output.iter().fold(0, |acc, x| acc + x.iter().filter(|c| **c == '#').count()));

    // part 2
    let mut output = input.clone();

    for (y, row) in input.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '.' || *c == '#' {
                continue;
            }

            if input.iter().fold(0, |acc, curr| acc + curr.iter().filter(|x| *x == c).count()) < 2 {
                continue;
            }
            for (yi, rowi) in input.iter().enumerate() {
                for (xi, ci) in rowi.iter().enumerate() {
                    if xi == x && yi == y {
                        continue;
                    }
                    if c == ci {
                        let dx = x as isize - xi as isize;
                        let dy = y as isize - yi as isize;

                        let mut poss_x = xi as isize;
                        let mut poss_y = yi as isize;


                        while poss_x >= 0 && poss_x < input[0].len() as isize && poss_y >= 0 && poss_y < input.len() as isize {
                            if output[poss_y as usize][poss_x as usize] != '#' {
                                output[poss_y as usize][poss_x as usize] = '#';
                            }
                            poss_x -= dx;
                            poss_y -= dy;
                        }
                    }
                }
            }
        }
    }

    dbg!(output.iter().fold(0, |acc, x| acc + x.iter().filter(|c| **c == '#').count()));


}
