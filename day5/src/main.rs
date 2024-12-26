fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();

    let (part_a, part_b) = input.split_once("\n\n")
                                .map(|(a, b)| (a.to_string(), b.to_string())).unwrap();

    let mut pairs: Vec<(isize, isize)> = Vec::new();

    for line in part_a.split_terminator('\n') {
        pairs.push(line.split_once("|").map(|(a, b)| (str::parse::<isize>(a).unwrap(), str::parse::<isize>(b).unwrap())).unwrap());
    }


    let mut result: isize = 0;

    for line in part_b.split_terminator("\n") {
        let nums: Vec<isize> = line.split(',').map(|a| str::parse::<isize>(a).unwrap()).collect();

        let mut ordered: bool = true;
        for pair in &pairs {
            if nums.contains(&pair.0) && nums.contains(&pair.1) && nums.iter().position(|&x| x == pair.0) > nums.iter().position(|&x| x == pair.1) {
                ordered = false;
                break;
            }
        }

        if ordered {
            result += nums[nums.len() / 2];
        }

    }

    dbg!(result);

    // part 2
    let mut result: isize = 0;
    for line in part_b.split_terminator("\n") {
        let mut nums: Vec<isize> = line.split(',').map(|a| str::parse::<isize>(a).unwrap()).collect();

        let mut already_ordered: bool = true;
        let mut ordered: bool = false;

        while !ordered {
            ordered = true;
            for pair in &pairs {
                if nums.contains(&pair.0) && nums.contains(&pair.1) && nums.iter().position(|&x| x == pair.0) > nums.iter().position(|&x| x == pair.1) {
                    let (i1, i2) = (nums.iter().position(|&x| x == pair.0).unwrap(), nums.iter().position(|&x| x == pair.1).unwrap());
                    nums.swap(i1, i2);
                    ordered = false;
                    already_ordered = false;
                    break;
                }
            }
        }

        if !already_ordered {
            result += nums[nums.len() / 2];
        }
    }

    dbg!(result);
}
