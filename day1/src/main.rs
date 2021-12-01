use day1::{count_increases, count_increases2};

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("no input file found");
    let m = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<i32>>();

    let result = count_increases(&m);
    println!("result 1: {}", result);

    let result = count_increases2(&m);
    println!("result 2: {}", result);
}
