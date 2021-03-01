const TREE: u8 = b'#';

fn trees_hit(lines: &Vec<&[u8]>, (x, y): (usize, usize)) -> usize {
    lines
        .iter()
        .enumerate()
        .skip(y)
        .filter(|(i, row)| i % y == 0 && row[((i / y) * x) % row.len()] == TREE)
        .count()
}

fn main() {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let lines = include_str!("../input.txt")
        .lines()
        .map(|row| row.as_bytes())
        .collect::<Vec<&[u8]>>();

    println!(
        "{}",
        slopes
            .iter()
            .map(|s| trees_hit(&lines, *s))
            .product::<usize>()
    );
}
