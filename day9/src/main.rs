#![feature(linked_list_remove)]

use std::collections::LinkedList;

fn main() {
    let mut input: LinkedList<u32> = std::fs::read_to_string("./input.txt").unwrap().chars().filter_map(|c| c.to_digit(10)).collect();
    let mut curr_top_file = input.len().div_ceil(2) - 1;
    let mut curr_bottom_file = 0;

    let mut blocks: Vec<usize> = Vec::new();
    let mut num_filled_end = input.pop_back().unwrap();

    while !input.is_empty() {
        let num_used = input.pop_front().unwrap();
        for _ in 0..num_used {
            blocks.push(curr_bottom_file.clone());
        }
        curr_bottom_file += 1;

        if input.is_empty() {
            break;
        }

        let mut num_free = input.pop_front().unwrap();

        while num_free > 0 {
            blocks.push(curr_top_file.clone());
            num_filled_end -= 1;
            num_free -= 1;

            if num_filled_end <= 0 {
                input.pop_back();
                num_filled_end = input.pop_back().unwrap();
                curr_top_file -= 1;
            }
        }
    }

    for _ in 0..num_filled_end {
        blocks.push(curr_top_file.clone());
    }

    let result = blocks.into_iter().enumerate().fold(0, |acc, (i, curr)| acc + i * curr);

    dbg!(result);

    // part 2

    let input: LinkedList<u32> = std::fs::read_to_string("./input.txt").unwrap().chars().filter_map(|c| c.to_digit(10)).collect();

    // filename, then number
    let mut filled: LinkedList<(usize, u32)> = LinkedList::new();
    let mut empty: Vec<u32> = Vec::new();

    // did this wrong way round, need to find leftmost free slot for rightmost file

    input.into_iter().enumerate().for_each(|(i, x)| {
        if i % 2 == 1 {
            empty.push(x);
        } else {
            filled.push_back((i / 2, x));
        }
    });

    empty.reverse();

    let mut blocks: Vec<usize> = Vec::new();

    while !filled.is_empty() {
        let (file, num) = filled.pop_front().unwrap();
        for _ in 0..num {
            blocks.push(file);
        }

        if !empty.is_empty() {
            let mut free_space = empty.pop().unwrap();
            while free_space > 0 {
                if let Some((i, (f, n))) = filled.clone().into_iter().enumerate().rfind(|(_,(_, n))| *n <= free_space) {
                    println!("Found {} size {} to fill {}", f, n, free_space);
                    for _ in 0..n {
                        blocks.push(f);
                    }
                    free_space -= n;
                    filled.remove(i);
                } else {
                    break;
                }
            }

            for _ in 0..free_space {
                dbg!("amogus");
                blocks.push(0);
            }
        }
    }

    println!("{:?}", blocks);

    let result = blocks.into_iter().enumerate().fold(0, |acc, (i, curr)| acc + i * curr);

    dbg!(result);
}
