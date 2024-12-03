use std::fs;

use regex::Regex;

fn main() {
    let input: String = fs::read_to_string("./input.txt").unwrap();
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Part 1
    dbg!(process(&input, &pattern));

    // Part 2
    let dos: Vec<&str> = input.split("do()").collect();
    let mut total: isize = 0;

    for s in dos {
        let actual = s.split_terminator("don't()").next().unwrap();
        total += process(actual, &pattern);
    }

    dbg!(total);
}

fn process(input: &str, pattern: &Regex) -> isize {
    pattern.captures_iter(input).map(|mt|(mt[1].parse().unwrap(), mt[2].parse().unwrap())).fold(0, |acc, curr: (isize, isize)| acc + curr.0 * curr.1)
}
