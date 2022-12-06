use std::cmp::max;
use std::path::Path;

pub fn puzzle1_1() {
    println!("{}", std::env::current_dir().unwrap().display());
    let contents = std::fs::read_to_string(Path::new("./src/inputs/puzzle1.txt")).unwrap();
    let bags = contents.split("\n\n").collect::<Vec<&str>>();

    let mut max_amount = 0;
    for bag in bags {
        let total_amount = bag
            .split("\n")
            .filter(|v| *v != "")
            .map(|v| v.parse::<i32>().unwrap())
            .reduce(|accum, v| accum + v)
            .unwrap();

        if total_amount > max_amount {
            max_amount = total_amount;
        }
    }

    println!("{:?}", max_amount);
}
