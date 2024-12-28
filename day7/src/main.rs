fn main() {
    let input: Vec<String> = std::fs::read_to_string("./input.txt").unwrap().split_terminator("\n").map(|l| l.to_owned()).collect();
    let mut total = 0;

    for line in &input {
        let (result, inputs) = line.split_once(": ").unwrap();
        let result = str::parse::<isize>(result).unwrap();
        let mut inputs: Vec<isize> = inputs.split(" ").map(|c| str::parse::<isize>(c).unwrap()).collect();
        inputs.reverse();

        if solvable(&result, &0, &mut inputs) {
            total += result;
        }

    }

    dbg!(total);

    // part 2
    let mut total = 0;
    for line in input {
        let (result, inputs) = line.split_once(": ").unwrap();
        let result = str::parse::<isize>(result).unwrap();
        let mut inputs: Vec<isize> = inputs.split(" ").map(|c| str::parse::<isize>(c).unwrap()).collect();
        inputs.reverse();

        if solvable_with_concat(&result, &0, &mut inputs) {
            total += result;
        }

    }

    dbg!(total);

}

fn solvable(target: &isize, prev: &isize, inputs: &mut Vec<isize>) -> bool {
    // println!("Trying to make {} with: {} and {:?}", target, prev, inputs.iter());
    if inputs.is_empty() {
        return target == prev;
    }

    let next = inputs.pop().unwrap();
    return solvable(target, &(prev + next), &mut inputs.clone()) || solvable(target, &(if *prev == 0 {1} else {*prev} * next), inputs);
}

fn solvable_with_concat(target: &isize, prev: &isize, inputs: &mut Vec<isize>) -> bool {
    // println!("Trying to make {} with: {} and {:?}", target, prev, inputs.iter());
    if inputs.is_empty() {
        return target == prev;
    }

    let next = inputs.pop().unwrap();

    if *prev == 0 {
        return solvable_with_concat(target, &next, inputs)
    }

    let mut concatted = prev.to_string();
    concatted.push_str(next.to_string().as_str());

    return solvable_with_concat(target, &(next + prev), &mut inputs.clone()) || solvable_with_concat(target, &(next * prev), &mut inputs.clone())
        || solvable_with_concat(target, &str::parse::<isize>(&concatted).unwrap(), inputs);
}
