use std::fs;

fn main() {
    let contents: String = fs::read_to_string("./input.txt").unwrap();
    let reports: Vec<&str> = contents.split_terminator('\n').collect();

    // part 1
    let mut count: usize = 0;
    for report in reports {
        let nums: Vec<usize> = report.split_terminator(' ').map(|x| x.parse().unwrap()).collect();
        if (nums.windows(2).all(|x| x[0] > x[1]) || nums.windows(2).all(|x| x[0] < x[1])) && nums.windows(2).all(|x| x[0].abs_diff(x[1]) <= 3) {
            count += 1;
        };
    }

    dbg!(count);

    // part 2
    count = 0;
    let reports: Vec<&str> = contents.split_terminator('\n').collect();
    for report in reports {
        let nums: Vec<usize> = report.split_terminator(' ').map(|x| x.parse().unwrap()).collect();
        if safe_damped_list(&nums) {
            count += 1;
        }
    }

    dbg!(count);
}

fn safe_damped_list(list: &Vec<usize>) -> bool {
    let mut damped: bool = false;
    let mut d: i8 = 0;

    for ns in list.windows(2) {
        if d == 0 {
            if ns[0] > ns[1] {
                d = -1;
            } else if ns[0] < ns[1] {
                d = 1;
            } else {
                damped = true;
            }
        } else if d == 1 {
            if ns[0] >= ns[1] {
                if damped {
                    return false;
                }
                damped = true;
            }
        } else {
            if ns[0] <= ns[1] {
                if damped {
                    return false;
                }
                damped = true;
            }
        }

        if ns[0].abs_diff(ns[1]) > 3 {
            if damped {
                return false;
            }
            damped = true;
        }
    }
    return true;
}
