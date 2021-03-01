use itertools::Itertools;

fn repair(input: &Vec<i64>, k: usize) -> i64 {
    input
        .iter()
        .cloned()
        .combinations(k)
        .filter(|i| i.iter().sum::<i64>() == 2020)
        .next()
        .map(|i| i.iter().product::<i64>())
        .unwrap()
}

fn main() {
    let input = include_str!("../input")
        .lines()
        .map(|i| i.parse::<i64>().unwrap())
        .sorted()
        .collect::<Vec<i64>>();

    println!("a {}", repair(&input, 2));
    println!("b {}", repair(&input, 3));
}
