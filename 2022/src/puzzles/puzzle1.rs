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

pub fn puzzle1_2() {
    println!("{}", std::env::current_dir().unwrap().display());
    let contents = std::fs::read_to_string(Path::new("./src/inputs/puzzle1.txt")).unwrap();
    let bags = contents.split("\n\n").collect::<Vec<&str>>();

    let mut total_bags = bags.iter()
        .map(|v| {
            let bag = *v;

            let total_amount = bag
                .split("\n")
                .filter(|v| *v != "")
                .map(|v| v.parse::<i32>().unwrap())
                .reduce(|accum, v| accum + v)
                .unwrap();

            total_amount
        })
        .collect::<Vec<i32>>();

    total_bags.sort_by(|a, b| b.cmp(a));

    println!("{}", total_bags[0] + total_bags[1] + total_bags[2])
}
