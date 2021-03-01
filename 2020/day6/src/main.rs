fn main() {
    let input = include_str!("../input.txt");

    println!(
        "anyone {}",
        input
            .split("\n\n")
            .map(|group| group.lines())
            .map(|rows| rows.fold(0, |gsum, row| gsum
                | row
                    .chars()
                    .fold(0, |rsum, c| rsum | (1u32 << c as u8 - b'a'))))
            .map(|c| c.count_ones())
            .sum::<u32>()
    );

    println!(
        "everyone {}",
        input
            .split("\n\n")
            .map(|group| group.lines())
            .map(|rows| rows.fold(u32::MAX, |gsum, row| gsum
                & row
                    .chars()
                    .fold(0, |rsum, c| rsum | (1u32 << c as u8 - b'a'))))
            .map(|c| c.count_ones())
            .sum::<u32>()
    );
}
