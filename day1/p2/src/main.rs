use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let mut file = File::open("./input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let pairs: Vec<&str> = contents.split_terminator('\n').collect();
    let mut l: Vec<u64> = Vec::new();
    let mut r: Vec<u64> = Vec::new();

    for pair in pairs {
        let mut items = pair.split("   ");
        l.push(items.next().unwrap().parse().unwrap());
        r.push(items.next().unwrap().parse().unwrap());
    }

    l.sort();
    r.sort();

    let mut sum: u64 = 0;
    l.iter().zip(&r).for_each(|(x, y)| sum += x.abs_diff(*y));

    println!("{}", sum);
    Ok(())
}
